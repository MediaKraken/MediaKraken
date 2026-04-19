use dioxus::prelude::*;

use crate::api::ApiClient;
use crate::models::{MediaItem, MediaKind};

#[derive(Clone, Debug, PartialEq)]
enum LibraryState {
    Idle,
    Loading,
    Loaded(Vec<MediaItem>),
    Error(String),
}

#[component]
pub fn MediaPage(client: ApiClient) -> Element {
    let mut kind = use_signal(|| MediaKind::Movie);
    let mut state = use_signal(|| LibraryState::Idle);

    let active = kind();
    let movies_class = tab_class(active == MediaKind::Movie);
    let shows_class = tab_class(active == MediaKind::Show);
    let audio_class = tab_class(active == MediaKind::Audio);

    let on_movies = make_loader(client.clone(), MediaKind::Movie, kind, state);
    let on_shows = make_loader(client.clone(), MediaKind::Show, kind, state);
    let on_audio = make_loader(client.clone(), MediaKind::Audio, kind, state);

    rsx! {
        section { class: "page",
            h2 { "Media library" }
            div { class: "tab-row",
                button { class: "{movies_class}", onclick: on_movies, "Movies" }
                button { class: "{shows_class}", onclick: on_shows, "TV Shows" }
                button { class: "{audio_class}", onclick: on_audio, "Audio" }
            }
            MediaListView { state: state(), kind: active }
        }
    }
}

fn make_loader(
    client: ApiClient,
    selected: MediaKind,
    mut kind: Signal<MediaKind>,
    mut state: Signal<LibraryState>,
) -> impl FnMut(MouseEvent) + 'static {
    move |_| {
        let client = client.clone();
        kind.set(selected);
        state.set(LibraryState::Loading);
        spawn(async move {
            let next = match client.list_media(selected).await {
                Ok(list) => LibraryState::Loaded(list.items),
                Err(message) => LibraryState::Error(message),
            };
            state.set(next);
        });
    }
}

fn tab_class(active: bool) -> &'static str {
    if active { "tab active" } else { "tab" }
}

#[component]
fn MediaListView(state: LibraryState, kind: MediaKind) -> Element {
    match state {
        LibraryState::Idle => rsx! {
            p { class: "muted", "Pick a category above to load {kind.label()}." }
        },
        LibraryState::Loading => rsx! {
            p { class: "muted", "Loading {kind.label()}..." }
        },
        LibraryState::Error(message) => rsx! {
            p { class: "error", "{message}" }
        },
        LibraryState::Loaded(items) => {
            if items.is_empty() {
                rsx! { p { class: "muted", "No {kind.label()} returned." } }
            } else {
                rsx! {
                    div { class: "media-grid",
                        for item in items {
                            MediaCard { key: "{item.id}", item: item }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MediaCard(item: MediaItem) -> Element {
    let year = item
        .year
        .map(|y| y.to_string())
        .unwrap_or_else(|| "—".to_string());
    let media_type = item.media_type.clone().unwrap_or_default();
    let overview = item.overview.clone().unwrap_or_default();

    rsx! {
        article { class: "media-card",
            if let Some(url) = item.poster_url.clone() {
                img { class: "media-poster", src: "{url}", alt: "{item.title}" }
            }
            div { class: "media-meta",
                h3 { "{item.title}" }
                p { class: "muted", "{media_type} · {year}" }
                if !overview.is_empty() {
                    p { "{overview}" }
                }
            }
        }
    }
}
