use dioxus::prelude::*;

use crate::api::ApiClient;
use crate::models::LibrarySummary;

#[derive(Clone, Debug, PartialEq)]
enum SummaryState {
    Idle,
    Loading,
    Loaded(LibrarySummary),
    Error(String),
}

#[component]
pub fn HomePage(client: ApiClient) -> Element {
    let mut state = use_signal(|| SummaryState::Idle);

    let load = {
        let client = client.clone();
        move |_| {
            let client = client.clone();
            spawn(async move {
                state.set(SummaryState::Loading);
                let next = match client.fetch_library_summary().await {
                    Ok(summary) => SummaryState::Loaded(summary),
                    Err(message) => SummaryState::Error(message),
                };
                state.set(next);
            });
        }
    };

    rsx! {
        section { class: "page",
            h2 { "Library summary" }
            p { "Verify connectivity to your MediaKraken server." }
            button { class: "primary", onclick: load, "Load library summary" }
            SummaryView { state: state() }
        }
    }
}

#[component]
fn SummaryView(state: SummaryState) -> Element {
    match state {
        SummaryState::Idle => rsx! {
            p { class: "muted", "Tap the button to fetch totals." }
        },
        SummaryState::Loading => rsx! {
            p { class: "muted", "Requesting library summary..." }
        },
        SummaryState::Loaded(summary) => rsx! {
            ul { class: "summary-list",
                li { "Server: {summary.server_name}" }
                li { "Version: {summary.version}" }
                li { "Movies: {summary.movie_count}" }
                li { "Shows: {summary.show_count}" }
                li { "Music albums: {summary.music_album_count}" }
            }
        },
        SummaryState::Error(message) => rsx! {
            p { class: "error", "{message}" }
        },
    }
}
