use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{db, markup};

pub async fn ws_handler(state: crate::AppState, ws: WebSocket) {
    let uuid = Uuid::new_v4();
    let (mut sink, mut stream) = ws.split();
    sink.send(Message::Text(
        markup::match_list(&db::get_matches(&mut state.pool.acquire().await.unwrap()).await)
            .into_string(),
    ))
    .await
    .unwrap();
    {
        let mut list = state.clients.0.write().await;
        println!("Connected ws: {uuid}");
        list.insert(
            uuid,
            Arc::new(Mutex::new(crate::Client {
                view: crate::ClientView::MainPage,
                sink,
            })),
        );
    }
    while let Some(_) = stream.next().await {}
    {
        let mut list = state.clients.0.write().await;
        println!("Disconnected ws: {uuid}");
        list.remove(&uuid);
    }
}

pub async fn match_ws_handler(id: i32, state: crate::AppState, ws: WebSocket) {
    let uuid = Uuid::new_v4();
    let (mut sink, mut stream) = ws.split();
    sink.send(Message::Text(
        markup::match_page_update(
            &db::get_match(&mut state.pool.acquire().await.unwrap(), id).await,
        )
        .into_string(),
    ))
    .await
    .unwrap();
    {
        let mut list = state.clients.0.write().await;
        println!("Connected match_ws({id}): {uuid}");
        list.insert(
            uuid,
            Arc::new(Mutex::new(crate::Client {
                view: crate::ClientView::MatchPage(id),
                sink,
            })),
        );
    }
    while let Some(_) = stream.next().await {}
    {
        let mut list = state.clients.0.write().await;
        println!("Disconnected match_ws({id}): {uuid}");
        list.remove(&uuid);
    }
}
