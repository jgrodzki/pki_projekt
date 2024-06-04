use crate::db;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use time::format_description;

pub fn index(body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html hx-ext="ws" {
            head {
                title {"VolleyballScoreboard"}
                meta charset="UTF-8";
                meta name="author" content="Jakub Grodzki 240675";
                meta name="viewport" content="width=device-width,initial-scale=1.0";
                link href="https://fonts.googleapis.com/css2?family=Inter:opsz@14..32&display=swap" rel="stylesheet";
                script src="https://unpkg.com/htmx.org@1.9.12/dist/htmx.min.js" {}
                script src="https://unpkg.com/htmx.org@1.9.12/dist/ext/ws.js" {}
                script src="https://cdn.tailwindcss.com" {}
                script {
                    (PreEscaped(include_str!("script.js")))
                }
            }
            body hx-target="this" .min-h-screen.text-white.bg-zinc-900.select-none.font-"[Inter]"."[&::-webkit-scrollbar]:bg-zinc-800"."[&::-webkit-scrollbar-thumb]:bg-sky-500"."[&::-webkit-scrollbar-thumb]:rounded-[min(0.357rem,0.714vw)]"."[&::-webkit-scrollbar]:w-[min(0.5rem,1vw)]" {
                (body)
            }
        }
    }
}

pub fn error(error_text: &str) -> Markup {
    html! {
        div #error hx-swap-oob="innerHTML" {
            div .fixed.w-screen.h-screen.top-0.left-0.z-50.flex.justify-center.text-"[min(1rem,2vw)]" {
                div .absolute.w-full.h-full.bg-"black/50".backdrop-blur-3xl onclick="close_error()" {}
                div .absolute.bg-zinc-800.rounded-"[min(0.357rem,0.714vw)]".w-"[min(24rem,48vw)]".top-"1/4".p-"[min(1rem,2vw)]".flex.flex-col.gap-"[min(1rem,2vw)]".justify-center.items-center.text-center.shadow-lg.shadow-"white/50" {
                    (error_text)
                    div .w-full.bg-sky-500.p-"[min(0.5rem,1vw)]".cursor-pointer.rounded-"[min(0.357rem,0.714vw)]"."hover:bg-sky-400"."active:bg-sky-300" onclick="close_error()" {"Ok"}
                }
            }
        }
    }
}

pub fn main_page() -> Markup {
    html! {
        (clipboard_def())
        div #error {}
        div ws-connect="/ws" hx-on:"htmx:ws-open"="matches_load()" hx-on:"htmx:ws-after-message"="matches_update()" .min-h-screen.max-w-6xl.mx-auto.text-"[min(1rem,2vw)]".bg-zinc-800.w-full.text-center.flex.flex-col {
            div .p-"[min(0.5rem,1vw)]".bg-zinc-800.fixed.top-0.w-full.max-w-6xl.flex.flex-col.gap-"[min(0.5rem,1vw)]" {
                div .flex.flex-none.gap-"[min(0.5rem,1vw)]" {
                    div #toggle_planned .w-full.flex-initial.bg-sky-500.p-"[min(0.5rem,1vw)]".cursor-pointer.rounded-"[min(0.357rem,0.714vw)]"."hover:bg-sky-400"."active:bg-sky-300" {
                        "Planned"
                    }
                    div #toggle_in_progress .w-full.flex-initial.bg-sky-500.p-"[min(0.5rem,1vw)]".cursor-pointer.rounded-"[min(0.357rem,0.714vw)]"."hover:bg-sky-400"."active:bg-sky-300" {
                        "InProgress"
                    }
                    div #toggle_finished .w-full.flex-initial.bg-sky-500.p-"[min(0.5rem,1vw)]".cursor-pointer.rounded-"[min(0.357rem,0.714vw)]"."hover:bg-sky-400"."active:bg-sky-300" {
                        "Finished"
                    }
                }
                div .bg-sky-500.flex.flex-none.items-center.rounded-"[min(0.357rem,0.714vw)]".py-"[min(1rem,2vw)]".px-"[min(0.5rem,1vw)]".gap-"[min(0.5rem,1vw)]" {
                    div .w-full.flex-initial {"Team 1"}
                    div .w-full.flex-initial {"Team 2"}
                    div .w-full.flex-initial {"Date"}
                    div .w-full.flex-initial {"Result"}
                    div .w-full.flex-initial {"Status"}
                    div .w-full.flex-initial {}
                }
            }
            div #match_list .px-"[min(0.5rem,1vw)]".pb-"[min(4.5rem,9vw)]".pt-"[min(7.5rem,15vw)]".text-center.flex.flex-col.gap-"[min(0.5rem,1vw)]" {}
            div #no_matches .px-"[min(0.5rem,1vw)]".pt-"[min(7.5rem,15vw)]".hidden {
                div .bg-zinc-700.flex.justify-center.items-center.rounded-"[min(0.357rem,0.714vw)]".py-"[min(1rem,2vw)]" {"No matches"}
            }
            div .fixed.bottom-0.w-full.max-w-6xl.p-"[min(0.5rem,1vw)]".bg-zinc-800 {
                form hx-post="/add_match" hx-swap="none" .grid.grid-cols-6.bg-zinc-700.rounded-"[min(0.357rem,0.714vw)]".p-"[min(0.5rem,1vw)]".gap-"[min(0.5rem,1vw)]" {
                    div .w-full.col-span-5.grid.grid-cols-3.gap-"[min(0.5rem,1vw)]".items-center {
                        input type="text" name="team_a" placeholder="Team 1 name" .p-"[min(0.5rem,1vw)]".placeholder-sky-500.outline-none."focus:outline-sky-500"."focus:outline-offset-0"."focus:outline"."focus:outline-[min(0.125rem,0.25vw)]".caret-sky-500.rounded-"[min(0.357rem,0.714vw)]".text-center.w-full.bg-zinc-800;
                        input type="text" name="team_b" placeholder="Team 2 name" .p-"[min(0.5rem,1vw)]".placeholder-sky-500.outline-none."focus:outline-sky-500"."focus:outline-offset-0"."focus:outline"."focus:outline-[min(0.125rem,0.25vw)]".caret-sky-500.rounded-"[min(0.357rem,0.714vw)]".text-center.w-full.bg-zinc-800;
                        input type="text" name="date" placeholder="Match date" .p-"[min(0.5rem,1vw)]".placeholder-sky-500.outline-none."focus:outline-sky-500"."focus:outline-offset-0"."focus:outline"."focus:outline-[min(0.125rem,0.25vw)]".caret-sky-500.rounded-"[min(0.357rem,0.714vw)]".text-center.w-full.bg-zinc-800;
                    }
                    div {
                        input type="submit" value="Add" .w-full.bg-sky-500.p-"[min(0.5rem,1vw)]".cursor-pointer.rounded-"[min(0.357rem,0.714vw)]"."hover:bg-sky-400"."active:bg-sky-300"."focus:outline-none"."focus:bg-sky-400";
                    }
                }
            }
        }
    }
}

pub fn match_page(match_id: i32) -> Markup {
    html! {
        div ws-connect={"/ws/" (match_id)} hx-on:":ws-after-message"="start_timer()" .h-screen.grid.justify-center.content-center {
            div #score .bg-zinc-800.h-"[min(100vh,calc(9/16*100vw))]".w-"[min(100vw,calc(16/9*100vh))]" {
            }
        }
    }
}

pub fn remove_match_page() -> Markup {
    html! {
        div #score hx-get="/" hx-trigger="load" {}
    }
}

pub fn match_page_update(match_info: &db::Match) -> Markup {
    html! {
        div #score .bg-zinc-800.h-"[min(100vh,calc(3/5*100vw))]".w-"[min(100vw,calc(5/3*100vh))]".flex.gap-"[min(2vh,calc(3/5*2vw))]".p-"[min(2vh,calc(3/5*2vw))]".text-"[min(4vh,calc(3/5*4vw))]".text-center {
            @if match_info.status == db::MatchStatus::InProgress {
                div #match_start .hidden {(match_info.match_start)}
                div #set_start .hidden {(match_info.set_start)}
            }
            div .basis-"2/5".w-full.h-full.flex.flex-col.gap-"[min(2vh,calc(3/5*2vw))]" {
                div .h-full.grid.justify-center.content-center {
                    div .truncate {
                        @if match_info.swapped {
                            (match_info.team_b)
                        } @else {
                            (match_info.team_a)
                        }
                    }
                }
                div .flex.flex-col.h-full.justify-end.items-start.gap-"[min(2vh,calc(3/5*2vw))]" {
                    @if match_info.status == db::MatchStatus::InProgress {
                        @if match_info.swapped {
                            div hx-post={"/remove_point_b/" (match_info.id)} hx-swap="none" .w-"1/3".aspect-square.bg-zinc-700.text-"[min(4vh,calc(3/5*4vw))]".rounded-"[min(0.714vh,calc(3/5*0.714vw))]".grid.justify-center.content-center.cursor-pointer."hover:bg-zinc-600"."active:bg-zinc-500" {"-1"}
                            div hx-post={"/add_point_b/" (match_info.id)} hx-swap="none" .w-full.aspect-square.bg-zinc-700.text-"[min(20vh,calc(3/5*20vw))]".rounded-"[min(0.714vh,calc(3/5*0.714vw))]".grid.justify-center.content-center.cursor-pointer."hover:bg-zinc-600"."active:bg-zinc-500" {(match_info.set_results_b.iter().last().unwrap())}
                        } @else {
                            div hx-post={"/remove_point_a/" (match_info.id)} hx-swap="none" .w-"1/3".aspect-square.bg-zinc-700.text-"[min(4vh,calc(3/5*4vw))]".rounded-"[min(0.714vh,calc(3/5*0.714vw))]".grid.justify-center.content-center.cursor-pointer."hover:bg-zinc-600"."active:bg-zinc-500" {"-1"}
                            div hx-post={"/add_point_a/" (match_info.id)} hx-swap="none" .w-full.aspect-square.bg-zinc-700.text-"[min(20vh,calc(3/5*20vw))]".rounded-"[min(0.714vh,calc(3/5*0.714vw))]".grid.justify-center.content-center.cursor-pointer."hover:bg-zinc-600"."active:bg-zinc-500" {(match_info.set_results_a.iter().last().unwrap())}
                        }
                    } @else {
                        div .w-"1/3".aspect-square.bg-zinc-900.text-"[min(4vh,calc(3/5*4vw))]".rounded-"[min(0.714vh,calc(3/5*0.714vw))]".grid.justify-center.content-center {"-1"}
                        div .w-full.aspect-square.bg-zinc-900.text-"[min(20vh,calc(3/5*20vw))]".rounded-"[min(0.714vh,calc(3/5*0.714vw))]".grid.justify-center.content-center {"0"}
                    }
                }
            }
            div .basis-"1/5".w-full.h-full.flex.flex-col.gap-"[min(2vh,calc(3/5*2vw))]" {
                div .flex.flex-row.justify-between.gap-"[min(2vh,calc(3/5*2vw))]" {
                    div .min-w-"[min(8vh,calc(3/5*8vw))]".grid.justify-center.content-center.bg-zinc-700.rounded-"[min(0.714vh,calc(3/5*0.714vw))]" {
                        @if match_info.swapped {
                            (match_info.result[1])
                        } @else {
                            (match_info.result[0])
                        }
                    }
                    @if match_info.status == db::MatchStatus::InProgress {
                        @if match_info.swapped {
                            div hx-post={"/swap_teams/" (match_info.id)} hx-swap="none" .w-full.h-"[min(8vh,calc(3/5*8vw))]".bg-sky-500.rounded-"[min(0.714vh,calc(3/5*0.714vw))]".grid.justify-center.content-center.cursor-pointer."hover:bg-sky-400"."active:bg-sky-300" {
                                div .size-"[min(4vh,calc(3/5*4vw))]" {(swap_icon())}
                            }
                        } @else {
                            div hx-post={"/swap_teams/" (match_info.id)} hx-swap="none" .w-full.h-"[min(8vh,calc(3/5*8vw))]".bg-zinc-700.rounded-"[min(0.714vh,calc(3/5*0.714vw))]".grid.justify-center.content-center.cursor-pointer."hover:bg-zinc-600"."active:bg-zinc-500" {
                                div .size-"[min(4vh,calc(3/5*4vw))]" {(swap_icon())}
                            }
                        }
                    } @else {
                        div .w-full.h-"[min(8vh,calc(3/5*8vw))]".bg-zinc-900.rounded-"[min(0.714vh,calc(3/5*0.714vw))]".grid.justify-center.content-center {
                            div .size-"[min(4vh,calc(3/5*4vw))]" {(swap_icon())}
                        }
                    }
                    @if match_info.swapped {
                        div .bg-zinc-700.min-w-"[min(8vh,calc(3/5*8vw))]".rounded-"[min(0.714vh,calc(3/5*0.714vw))]".grid.justify-center.content-center {(match_info.result[0])}

                    } @else {
                        div .bg-zinc-700.min-w-"[min(8vh,calc(3/5*8vw))]".rounded-"[min(0.714vh,calc(3/5*0.714vw))]".grid.justify-center.content-center {(match_info.result[1])}
                    }
                }
                div .flex.flex-col.justify-between.h-full.gap-"[min(2vh,calc(3/5*2vw))]" {
                    div .h-full.flex.flex-col.gap-"[min(2vh,calc(3/5*2vw))]" {
                        div .rounded-"[min(0.714vh,calc(3/5*0.714vw))]".overflow-hidden {
                            div .bg-sky-500 {"Match status"}
                            div .bg-zinc-700 {(format!("{:?}",match_info.status))}
                        }
                        @if match_info.status == db::MatchStatus::InProgress {
                            div .rounded-"[min(0.714vh,calc(3/5*0.714vw))]".overflow-hidden {
                                div .bg-sky-500 {"Current time"}
                                div .bg-zinc-700 #current_time {}
                            }
                            div .rounded-"[min(0.714vh,calc(3/5*0.714vw))]".overflow-hidden {
                                div .bg-sky-500 {"Match time"}
                                div .bg-zinc-700 #match_time {}
                            }
                            div .rounded-"[min(0.714vh,calc(3/5*0.714vw))]".overflow-hidden {
                                div .bg-sky-500 {"Set time"}
                                div .bg-zinc-700 #set_time {}
                            }
                        }
                    }
                    div .flex.flex-col.gap-"[min(2vh,calc(3/5*2vw))]" {
                        @match button_status(&match_info) {
                            ButtonStatus::Start => {
                                div hx-post={"/end_set/" (match_info.id)} hx-swap="none" .text-center.block.w-full.bg-sky-500.p-"[min(2vh,calc(3/5*2vw))]".cursor-pointer.rounded-"[min(0.714vh,calc(3/5*0.714vw))]"."hover:bg-sky-400"."active:bg-sky-300" {"Start match"}
                            }
                            ButtonStatus::EndSet => {
                                div hx-post={"/end_set/" (match_info.id)} hx-swap="none" .text-center.block.w-full.bg-sky-500.p-"[min(2vh,calc(3/5*2vw))]".cursor-pointer.rounded-"[min(0.714vh,calc(3/5*0.714vw))]"."hover:bg-sky-400"."active:bg-sky-300" {"End set"}
                            }
                            ButtonStatus::EndMatch => {
                                div hx-post={"/end_set/" (match_info.id)} hx-swap="none" .text-center.block.w-full.bg-sky-500.p-"[min(2vh,calc(3/5*2vw))]".cursor-pointer.rounded-"[min(0.714vh,calc(3/5*0.714vw))]"."hover:bg-sky-400"."active:bg-sky-300" {"End match"}
                            }
                            ButtonStatus::None => {
                            },
                        }
                        a href="/" hx-boost="true" .text-center.block.w-full.bg-sky-500.p-"[min(2vh,calc(3/5*2vw))]".cursor-pointer.rounded-"[min(0.714vh,calc(3/5*0.714vw))]"."hover:bg-sky-400"."active:bg-sky-300"."focus:outline-none"."focus:bg-sky-400" {
                            "Back"
                        }
                    }
                }
            }
            div .basis-"2/5".w-full.h-full.flex.flex-col.gap-"[min(2vh,calc(3/5*2vw))]" {
                div .h-full.grid.justify-center.content-center {
                    div .truncate {
                        @if match_info.swapped {
                            (match_info.team_a)
                        } @else {
                            (match_info.team_b)
                        }
                    }
                }
                div .flex.flex-col.h-full.justify-end.items-end.gap-"[min(2vh,calc(3/5*2vw))]" {
                    @if match_info.status == db::MatchStatus::InProgress {
                        @if match_info.swapped {
                            div hx-post={"/remove_point_a/" (match_info.id)} hx-swap="none" .w-"1/3".aspect-square.bg-zinc-700.text-"[min(4vh,calc(3/5*4vw))]".rounded-"[min(0.714vh,calc(3/5*0.714vw))]".grid.justify-center.content-center.cursor-pointer."hover:bg-zinc-600"."active:bg-zinc-500" {"-1"}
                            div hx-post={"/add_point_a/" (match_info.id)} hx-swap="none" .w-full.aspect-square.bg-zinc-700.text-"[min(20vh,calc(3/5*20vw))]".rounded-"[min(0.714vh,calc(3/5*0.714vw))]".grid.justify-center.content-center.cursor-pointer."hover:bg-zinc-600"."active:bg-zinc-500" {(match_info.set_results_a.iter().last().unwrap())}
                        } @else {
                            div hx-post={"/remove_point_b/" (match_info.id)} hx-swap="none" .w-"1/3".aspect-square.bg-zinc-700.text-"[min(4vh,calc(3/5*4vw))]".rounded-"[min(0.714vh,calc(3/5*0.714vw))]".grid.justify-center.content-center.cursor-pointer."hover:bg-zinc-600"."active:bg-zinc-500" {"-1"}
                            div hx-post={"/add_point_b/" (match_info.id)} hx-swap="none" .w-full.aspect-square.bg-zinc-700.text-"[min(20vh,calc(3/5*20vw))]".rounded-"[min(0.714vh,calc(3/5*0.714vw))]".grid.justify-center.content-center.cursor-pointer."hover:bg-zinc-600"."active:bg-zinc-500" {(match_info.set_results_b.iter().last().unwrap())}
                        }
                    } @else {
                        div .w-"1/3".aspect-square.bg-zinc-900.text-"[min(4vh,calc(3/5*4vw))]".rounded-"[min(0.714vh,calc(3/5*0.714vw))]".grid.justify-center.content-center {"-1"}
                        div .w-full.aspect-square.bg-zinc-900.text-"[min(20vh,calc(3/5*20vw))]".rounded-"[min(0.714vh,calc(3/5*0.714vw))]".grid.justify-center.content-center {"0"}
                    }
                }
            }
        }
    }
}

fn score_print(match_info: &db::Match) -> Markup {
    let number_len = |&n: &i32| if n == 0 { 0 } else { n.ilog10() + 1 };
    let score_padding = match_info
        .set_results_a
        .iter()
        .map(number_len)
        .max()
        .unwrap()
        .max(
            match_info
                .set_results_b
                .iter()
                .map(number_len)
                .max()
                .unwrap(),
        ) as usize;
    let name_padding = match_info
        .team_a
        .chars()
        .count()
        .max(match_info.team_b.chars().count())
        + 1;
    let mut result = format!("{:<1$}", "", name_padding);
    for set_number in 1..=match_info.set_results_a.len() {
        result += &format!("S{:<1$} | ", set_number, score_padding - 1);
    }
    result += &format!("Total\\n{:<1$}", match_info.team_a, name_padding);
    for set_score in &match_info.set_results_a {
        result += &format!("{:<1$} | ", set_score, score_padding);
    }
    result += &format!(
        "{:^5}\\n{:<2$}",
        match_info.result[0], match_info.team_b, name_padding
    );
    for set_score in &match_info.set_results_b {
        result += &format!("{:<1$} | ", set_score, score_padding);
    }
    let format = format_description::parse("[year].[month].[day] [hour]:[minute]").unwrap();
    result += &format!(
        "{:^5}\\n{}",
        match_info.result[1],
        match_info.match_start.format(&format).unwrap()
    );
    PreEscaped(result)
}

pub fn add_match_entry(match_info: &db::Match) -> Markup {
    html! {
        div #match_list hx-swap-oob="beforeend" {
            (match_entry(match_info))
        }
    }
}

pub fn update_match_entry(match_info: &db::Match) -> Markup {
    html! {
        (match_entry(match_info))
    }
}

pub fn remove_match_entry(match_id: i32) -> Markup {
    html! {
        div #{"match_" (match_id)} hx-swap-oob="delete" {}
    }
}

fn match_entry(match_info: &db::Match) -> Markup {
    let format = format_description::parse("[year].[month].[day] [hour]:[minute]").unwrap();
    html! {
        div #{"match_" (match_info.id)} hx-get={"/match/" (match_info.id)} hx-push-url="true" hx-disinherit="hx-push-url" .flex-none.bg-zinc-"700"."hover:bg-zinc-600"."[&:active:not(:has(.ignore:active))]:bg-zinc-500".overflow-hidden.flex.items-center.gap-"[min(0.5rem,1vw)]".rounded-"[min(0.357rem,0.714vw)]".px-"[min(0.5rem,1vw)]" {
            div .w-full.flex-initial.truncate {
                (match_info.team_a)
            }
            div .w-full.flex-initial.truncate {
                (match_info.team_b)
            }
            div .w-full.flex-initial.min-w-max {
                (match_info.match_start.format(&format).unwrap())
            }
            @if match_info.status == db::MatchStatus::Finished {
                div onclick={"handle_clipboard(event,'" (score_print(match_info)) "')"} .ignore.cursor-pointer.w-full.flex-initial.self-stretch.flex.justify-center.items-center.gap-"[min(0.25rem,0.5vw)]" {
                    (format!("{}:{}",match_info.result[0],match_info.result[1]))
                    div .clipboard.h-"[min(1rem,2vw)]".self-center {
                        svg viewBox="0 0 24 24" .size-full {
                            use xlink:href="#clipboard-icon";
                        }
                    }
                }
            }
            @else {
                div .w-full.flex-initial {
                    (format!("{}:{}",match_info.result[0],match_info.result[1]))
                }
            }
            div .w-full.flex-initial {
                (format!("{:?}",match_info.status))
            }
            div .w-full.flex-initial {
                div hx-trigger="click consume" hx-post={"/remove_match/" (match_info.id)} hx-swap="none" .ignore.bg-sky-500.p-"[min(0.5rem,1vw)]".my-"[min(0.5rem,1vw)]".cursor-pointer.rounded-"[min(0.357rem,0.714vw)]"."hover:bg-sky-400"."active:bg-sky-300" {
                  "Delete"
                }
            }
        }
    }
}

pub fn match_list(matches: &[db::Match]) -> Markup {
    html! {
        div #match_list hx-swap-oob="innerHTML" {
            @for match_info in matches {
                (match_entry(match_info))
            }
        }
    }
}

fn swap_icon() -> Markup {
    html! {
        svg fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-full" {
            path stroke-linecap="round" stroke-linejoin="round" d="M7.5 21 3 16.5m0 0L7.5 12M3 16.5h13.5m0-13.5L21 7.5m0 0L16.5 12M21 7.5H7.5";

        }
    }
}

fn clipboard_def() -> Markup {
    html! {
        svg class="hidden" {
            defs {
                path #clipboard-check-icon fill="none" stroke-width="1.5" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" d="M11.35 3.836c-.065.21-.1.433-.1.664 0 .414.336.75.75.75h4.5a.75.75 0 0 0 .75-.75 2.25 2.25 0 0 0-.1-.664m-5.8 0A2.251 2.251 0 0 1 13.5 2.25H15c1.012 0 1.867.668 2.15 1.586m-5.8 0c-.376.023-.75.05-1.124.08C9.095 4.01 8.25 4.973 8.25 6.108V8.25m8.9-4.414c.376.023.75.05 1.124.08 1.131.094 1.976 1.057 1.976 2.192V16.5A2.25 2.25 0 0 1 18 18.75h-2.25m-7.5-10.5H4.875c-.621 0-1.125.504-1.125 1.125v11.25c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V18.75m-7.5-10.5h6.375c.621 0 1.125.504 1.125 1.125v9.375m-8.25-3 1.5 1.5 3-3.75";
                path #clipboard-icon stroke-width="1.5" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" d="M9 12h3.75M9 15h3.75M9 18h3.75m3 .75H18a2.25 2.25 0 0 0 2.25-2.25V6.108c0-1.135-.845-2.098-1.976-2.192a48.424 48.424 0 0 0-1.123-.08m-5.801 0c-.065.21-.1.433-.1.664 0 .414.336.75.75.75h4.5a.75.75 0 0 0 .75-.75 2.25 2.25 0 0 0-.1-.664m-5.8 0A2.251 2.251 0 0 1 13.5 2.25H15c1.012 0 1.867.668 2.15 1.586m-5.8 0c-.376.023-.75.05-1.124.08C9.095 4.01 8.25 4.973 8.25 6.108V8.25m0 0H4.875c-.621 0-1.125.504-1.125 1.125v11.25c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V9.375c0-.621-.504-1.125-1.125-1.125H8.25ZM6.75 12h.008v.008H6.75V12Zm0 3h.008v.008H6.75V15Zm0 3h.008v.008H6.75V18Z";
            }
        }
    }
}

enum ButtonStatus {
    Start,
    EndSet,
    EndMatch,
    None,
}

fn button_status(match_info: &db::Match) -> ButtonStatus {
    if match_info.status == db::MatchStatus::Finished {
        return ButtonStatus::None;
    }
    if match_info.status == db::MatchStatus::Planned {
        return ButtonStatus::Start;
    }
    let set_points_a = *match_info.set_results_a.last().unwrap();
    let set_points_b = *match_info.set_results_b.last().unwrap();
    if match_info.result[0] == 2 && match_info.result[1] == 2 {
        if (set_points_a < 15 && set_points_b < 15) || set_points_a.abs_diff(set_points_b) < 2 {
            return ButtonStatus::None;
        }
    } else {
        if (set_points_a < 25 && set_points_b < 25) || set_points_a.abs_diff(set_points_b) < 2 {
            return ButtonStatus::None;
        }
    }
    let result_index = if set_points_a > set_points_b { 0 } else { 1 };
    if match_info.result[result_index] >= 2 {
        ButtonStatus::EndMatch
    } else {
        ButtonStatus::EndSet
    }
}
