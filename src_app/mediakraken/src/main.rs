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

#[cfg(any(target_os = "android", target_os = "ios"))]
fn main() {
    dioxus_mobile::launch(app);
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let base_url = use_state(cx, || "https://mediakraken.example.com".to_string());
    let bearer_token = use_state(cx, String::new);
    let api_state = use_state(cx, || ApiState::Idle);

    cx.render(rsx! {
        main {
            class: "app-shell",
            h1 { "MediaKraken mobile starter" }
            p {
                "Use this Dioxus screen as a starting point for Android/iOS clients that talk to the MediaKraken API."
            }
            div {
                class: "form-grid",
                label {
                    "API base URL"
                    input {
                        value: "{base_url}",
                        oninput: move |event| base_url.set(event.value.clone()),
                        placeholder: "https://mediakraken.example.com",
                    }
                }
                label {
                    "Bearer token"
                    input {
                        r#type: "password",
                        value: "{bearer_token}",
                        oninput: move |event| bearer_token.set(event.value.clone()),
                        placeholder: "Optional API token",
                    }
                }
                button {
                    onclick: move |_| {
                        let base_url_value = base_url.current().clone();
                        let bearer_token_value = bearer_token.current().clone();
                        let api_state = api_state.clone();

                        cx.spawn(async move {
                            api_state.set(ApiState::Loading);

                            let next_state = match ApiClient::new(base_url_value, bearer_token_value)
                                .fetch_library_summary()
                                .await
                            {
                                Ok(summary) => ApiState::Loaded(summary),
                                Err(error) => ApiState::Error(error.to_string()),
                            };

                            api_state.set(next_state);
                        });
                    },
                    "Load library summary"
                }
            }
            {render_api_state(cx, api_state.current())}
        }
    })
}

fn render_api_state<'a>(cx: Scope<'a>, api_state: &'a ApiState) -> Element<'a> {
    match api_state {
        ApiState::Idle => cx.render(rsx! {
            section {
                h2 { "Ready" }
                p { "Set the API URL above and tap the button to verify connectivity." }
            }
        }),
        ApiState::Loading => cx.render(rsx! {
            section {
                h2 { "Loading" }
                p { "Requesting MediaKraken library summary..." }
            }
        }),
        ApiState::Loaded(summary) => cx.render(rsx! {
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
        }),
        ApiState::Error(message) => cx.render(rsx! {
            section {
                h2 { "Request failed" }
                p { "{message}" }
                p { "Confirm the base URL, authentication token, and route path match your server." }
            }
        }),
    }
}
