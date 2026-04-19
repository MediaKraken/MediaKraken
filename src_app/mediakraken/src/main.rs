mod api;
mod models;

use dioxus::prelude::*;

use crate::api::ApiClient;
use crate::models::LibrarySummary;

#[derive(Clone, Debug, PartialEq)]
enum ApiState {
    Idle,
    Loading,
    Loaded(LibrarySummary),
    Error(String),
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut base_url = use_signal(|| "https://www.mediakraken.media".to_string());
    let mut bearer_token = use_signal(String::new);
    let mut api_state = use_signal(|| ApiState::Idle);
    let http_client = use_signal(ApiClient::default_http_client);

    let load_summary = move |_| {
        let client = ApiClient::from_parts(http_client(), base_url(), bearer_token());
        spawn(async move {
            api_state.set(ApiState::Loading);
            let next = match client.fetch_library_summary().await {
                Ok(summary) => ApiState::Loaded(summary),
                Err(message) => ApiState::Error(message),
            };
            api_state.set(next);
        });
    };

    rsx! {
        main { class: "app-shell",
            h1 { "MediaKraken" }
            div { class: "form-grid",
                label { "API base URL"
                    input {
                        value: "{base_url}",
                        oninput: move |event| base_url.set(event.value()),
                        placeholder: "https://www.mediakraken.media",
                        autocomplete: "off",
                    }
                }
                label { "Bearer token"
                    input {
                        r#type: "password",
                        value: "{bearer_token}",
                        oninput: move |event| bearer_token.set(event.value()),
                        placeholder: "Optional API token",
                        autocomplete: "off",
                    }
                }
                button { onclick: load_summary, "Load library summary" }
            }
            ApiStateView { state: api_state() }
        }
    }
}

#[component]
fn ApiStateView(state: ApiState) -> Element {
    match state {
        ApiState::Idle => rsx! {
            section {
                h2 { "Ready" }
                p { "Set the API URL above and tap the button to verify connectivity." }
            }
        },
        ApiState::Loading => rsx! {
            section {
                h2 { "Loading" }
                p { "Requesting MediaKraken library summary..." }
            }
        },
        ApiState::Loaded(summary) => rsx! {
            section {
                h2 { "Connected" }
                ul {
                    li { "Server: {summary.server_name}" }
                    li { "Version: {summary.version}" }
                    li { "Movies: {summary.movie_count}" }
                    li { "Shows: {summary.show_count}" }
                    li { "Music albums: {summary.music_album_count}" }
                }
            }
        },
        ApiState::Error(message) => rsx! {
            section {
                h2 { "Request failed" }
                p { "{message}" }
                p { "Confirm the base URL, authentication token, and route path match your server." }
            }
        },
    }
}
