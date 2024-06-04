use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Form, Router,
};
use axum_htmx::HxBoosted;
use dotenvy::dotenv;
use futures_util::{stream::SplitSink, SinkExt};
use serde::Deserialize;
use sqlx::PgPool;
use std::{collections::HashMap, env, sync::Arc};
use tokio::{
    net::TcpListener,
    sync::{Mutex, RwLock},
};
use uuid::Uuid;

mod db;
mod markup;
mod ws;

async fn index_handler(HxBoosted(boosted): HxBoosted) -> impl IntoResponse {
    let body = markup::main_page();
    if boosted {
        body
    } else {
        markup::index(body)
    }
}

#[derive(Deserialize)]
struct AddMatchForm {
    date: String,
    team_a: String,
    team_b: String,
}

async fn add_match_handler(
    State(state): State<AppState>,
    Form(form): Form<AddMatchForm>,
) -> impl IntoResponse {
    match db::add_match(
        &mut state.pool.acquire().await.unwrap(),
        &form.team_a,
        &form.team_b,
        &form.date,
    )
    .await
    {
        Ok(match_info) => {
            state
                .clients
                .send_to_clients(
                    ClientView::MainPage,
                    &Message::Text(markup::add_match_entry(&match_info).into_string()),
                )
                .await;
            ().into_response()
        }
        Err(e) => return markup::error(&e.to_string()).into_response(),
    }
}

async fn swap_teams_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let mut tx = state.pool.begin().await.unwrap();
    if db::swap_teams(&mut *tx, id).await {
        state
            .clients
            .send_to_clients(
                ClientView::MatchPage(id),
                &Message::Text(
                    markup::match_page_update(&db::get_match(&mut *tx, id).await).into_string(),
                ),
            )
            .await;
        tx.commit().await.unwrap();
    }
}

async fn match_handler(
    State(state): State<AppState>,
    path: Option<Path<i32>>,
    HxBoosted(boosted): HxBoosted,
) -> impl IntoResponse {
    let Some(Path(id)) = path else {
        return StatusCode::NOT_FOUND.into_response();
    };
    if !db::match_exists(&mut state.pool.acquire().await.unwrap(), id).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    let body = markup::match_page(id);
    if boosted {
        body.into_response()
    } else {
        markup::index(body).into_response()
    }
}

async fn remove_match_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    if db::remove_match(&mut state.pool.acquire().await.unwrap(), id).await {
        state
            .clients
            .send_to_clients(
                ClientView::MainPage,
                &Message::Text(markup::remove_match_entry(id).into_string()),
            )
            .await;
        state
            .clients
            .send_to_clients(
                ClientView::MatchPage(id),
                &Message::Text(markup::remove_match_page().into_string()),
            )
            .await;
    }
}

async fn add_point_a_handler(State(state): State<AppState>, Path(id): Path<i32>) {
    let mut tx = state.pool.begin().await.unwrap();
    if db::add_set_point_a(&mut *tx, id).await {
        state
            .clients
            .send_to_clients(
                ClientView::MatchPage(id),
                &Message::Text(
                    markup::match_page_update(&db::get_match(&mut *tx, id).await).into_string(),
                ),
            )
            .await;
        tx.commit().await.unwrap();
    }
}

async fn add_point_b_handler(State(state): State<AppState>, Path(id): Path<i32>) {
    let mut tx = state.pool.begin().await.unwrap();
    if db::add_set_point_b(&mut *tx, id).await {
        state
            .clients
            .send_to_clients(
                ClientView::MatchPage(id),
                &Message::Text(
                    markup::match_page_update(&db::get_match(&mut *tx, id).await).into_string(),
                ),
            )
            .await;
        tx.commit().await.unwrap();
    }
}

async fn remove_point_a_handler(State(state): State<AppState>, Path(id): Path<i32>) {
    let mut tx = state.pool.begin().await.unwrap();
    if db::remove_set_point_a(&mut tx, id).await {
        state
            .clients
            .send_to_clients(
                ClientView::MatchPage(id),
                &Message::Text(
                    markup::match_page_update(&db::get_match(&mut *tx, id).await).into_string(),
                ),
            )
            .await;
        tx.commit().await.unwrap();
    }
}

async fn remove_point_b_handler(State(state): State<AppState>, Path(id): Path<i32>) {
    let mut tx = state.pool.begin().await.unwrap();
    if db::remove_set_point_b(&mut *tx, id).await {
        state
            .clients
            .send_to_clients(
                ClientView::MatchPage(id),
                &Message::Text(
                    markup::match_page_update(&db::get_match(&mut *tx, id).await).into_string(),
                ),
            )
            .await;
        tx.commit().await.unwrap();
    }
}

async fn end_set_handler(State(state): State<AppState>, Path(id): Path<i32>) {
    let mut tx = state.pool.begin().await.unwrap();
    if db::end_set(&mut *tx, id).await {
        let match_info = db::get_match(&mut *tx, id).await;
        tx.commit().await.unwrap();
        state
            .clients
            .send_to_clients(
                ClientView::MatchPage(id),
                &Message::Text(markup::match_page_update(&match_info).into_string()),
            )
            .await;
        state
            .clients
            .send_to_clients(
                ClientView::MainPage,
                &Message::Text(markup::update_match_entry(&match_info).into_string()),
            )
            .await;
    }
}

async fn ws_upgrade_handler(
    State(state): State<AppState>,
    wsu: WebSocketUpgrade,
) -> impl IntoResponse {
    wsu.on_upgrade(|ws| ws::ws_handler(state, ws))
}

async fn match_ws_upgrade_handler(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    wsu: WebSocketUpgrade,
) -> impl IntoResponse {
    wsu.on_upgrade(move |ws| ws::match_ws_handler(id, state, ws))
}

#[derive(PartialEq, Eq)]
enum ClientView {
    MainPage,
    MatchPage(i32),
}

struct Client {
    view: ClientView,
    sink: SplitSink<WebSocket, Message>,
}

#[derive(Default, Clone)]
struct ClientList(Arc<RwLock<HashMap<Uuid, Arc<Mutex<Client>>>>>);

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    clients: ClientList,
}

impl ClientList {
    async fn send_to_clients(&self, view: ClientView, message: &Message) {
        for client in self.0.write().await.values() {
            let mut client = client.clone().lock_owned().await;

            if client.view == view {
                let message = message.clone();
                tokio::spawn(async move { client.sink.send(message).await.unwrap() });
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().unwrap();
    let pool = PgPool::connect_lazy(&env::var("DATABASE_URL").unwrap()).unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/ws", get(ws_upgrade_handler))
        .route("/ws/:id", get(match_ws_upgrade_handler))
        .route("/add_match", post(add_match_handler))
        .route("/remove_match/:id", post(remove_match_handler))
        .route("/add_point_a/:id", post(add_point_a_handler))
        .route("/add_point_b/:id", post(add_point_b_handler))
        .route("/remove_point_a/:id", post(remove_point_a_handler))
        .route("/remove_point_b/:id", post(remove_point_b_handler))
        .route("/swap_teams/:id", post(swap_teams_handler))
        .route("/end_set/:id", post(end_set_handler))
        .route("/match/:id", get(match_handler))
        .with_state(AppState {
            pool,
            clients: ClientList::default(),
        });
    let listener = TcpListener::bind("0.0.0.0:".to_owned() + &env::var("PORT").unwrap())
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
