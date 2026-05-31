mod api;
mod models;
mod pages;

use dioxus::prelude::*;

use crate::api::ApiClient;
use crate::pages::hardware::HardwarePage;
use crate::pages::home::HomePage;
use crate::pages::media::MediaPage;
use crate::pages::scan::ScanPage;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Page {
    Home,
    Scan,
    Hardware,
    Media,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut base_url = use_signal(|| "https://www.mediakraken.media".to_string());
    let mut bearer_token = use_signal(String::new);
    let mut page = use_signal(|| Page::Home);
    let http_client = use_signal(ApiClient::default_http_client);

    let client = ApiClient::from_parts(http_client(), base_url(), bearer_token());

    rsx! {
        head {
            title { "MediaKraken - Multimedia Player" }
            style { {include_str!("styles/main.css")} }
        }
        main { class: "app-shell",
            header { class: "app-header",
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
                }
            }

            nav { class: "tab-row",
                button {
                    class: "{tab_class(page() == Page::Home)}",
                    onclick: move |_| page.set(Page::Home),
                    "Home"
                }
                button {
                    class: "{tab_class(page() == Page::Scan)}",
                    onclick: move |_| page.set(Page::Scan),
                    "Scan"
                }
                button {
                    class: "{tab_class(page() == Page::Hardware)}",
                    onclick: move |_| page.set(Page::Hardware),
                    "Hardware"
                }
                button {
                    class: "{tab_class(page() == Page::Media)}",
                    onclick: move |_| page.set(Page::Media),
                    "Media"
                }
            }

            match page() {
                Page::Home => rsx! { HomePage { client: client.clone() } },
                Page::Scan => rsx! { ScanPage { client: client.clone() } },
                Page::Hardware => rsx! { HardwarePage { client: client.clone() } },
                Page::Media => rsx! { MediaPage { client: client.clone() } },
            }
        }
    }
}

fn tab_class(active: bool) -> &'static str {
    if active { "tab active" } else { "tab" }
}
