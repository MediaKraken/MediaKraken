use dioxus::prelude::*;

use crate::api::ApiClient;
use crate::models::UpcLookupResult;

#[derive(Clone, Debug, PartialEq)]
enum ScanState {
    Idle,
    Looking,
    Found(UpcLookupResult),
    Error(String),
}

#[component]
pub fn ScanPage(client: ApiClient) -> Element {
    let mut upc = use_signal(String::new);
    let mut state = use_signal(|| ScanState::Idle);

    let lookup = {
        let client = client.clone();
        move |_| {
            let client = client.clone();
            let code = upc();
            spawn(async move {
                state.set(ScanState::Looking);
                let next = match client.lookup_upc(&code).await {
                    Ok(result) => ScanState::Found(result),
                    Err(message) => ScanState::Error(message),
                };
                state.set(next);
            });
        }
    };

    rsx! {
        section { class: "page",
            h2 { "Scan UPC" }
            p { class: "muted",
                "Use the device camera to capture a barcode photo, then enter the UPC digits to check if you already own the title."
            }
            div { class: "form-grid",
                label { "Camera capture"
                    input {
                        r#type: "file",
                        accept: "image/*",
                        "capture": "environment",
                    }
                }
                label { "UPC code"
                    input {
                        r#type: "text",
                        "inputmode": "numeric",
                        value: "{upc}",
                        oninput: move |event| upc.set(event.value()),
                        placeholder: "e.g. 786936224290",
                        autocomplete: "off",
                    }
                }
                button { class: "primary", onclick: lookup, "Check ownership" }
            }
            ScanResultView { state: state() }
        }
    }
}

#[component]
fn ScanResultView(state: ScanState) -> Element {
    match state {
        ScanState::Idle => rsx! {
            p { class: "muted", "Awaiting a UPC code." }
        },
        ScanState::Looking => rsx! {
            p { class: "muted", "Asking the server..." }
        },
        ScanState::Found(result) => {
            let title = result.title.clone().unwrap_or_else(|| "Unknown title".to_string());
            let media_type = result
                .media_type
                .clone()
                .unwrap_or_else(|| "unknown".to_string());
            let year = result
                .year
                .map(|y| y.to_string())
                .unwrap_or_else(|| "—".to_string());
            let status = if result.owned { "Already in library" } else { "Not in library" };
            let class = if result.owned { "result owned" } else { "result missing" };
            rsx! {
                div { class: "{class}",
                    h3 { "{status}" }
                    ul {
                        li { "UPC: {result.upc}" }
                        li { "Title: {title}" }
                        li { "Type: {media_type}" }
                        li { "Year: {year}" }
                    }
                }
            }
        }
        ScanState::Error(message) => rsx! {
            p { class: "error", "{message}" }
        },
    }
}
