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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackMode {
    Movie,
    Show,
    Audio,
}

#[component]
pub fn MediaPage(client: ApiClient) -> Element {
    let mut kind = use_signal(|| MediaKind::Movie);
    let mut state = use_signal(|| LibraryState::Idle);
    let mut playback_item = use_signal(|| None::<(MediaItem, MediaKind)>);

    let active = kind();
    let movies_class = tab_class(active == MediaKind::Movie);
    let shows_class = tab_class(active == MediaKind::Show);
    let audio_class = tab_class(active == MediaKind::Audio);

    let on_movies = make_loader(client.clone(), MediaKind::Movie, kind, state);
    let on_shows = make_loader(client.clone(), MediaKind::Show, kind, state);
    let on_audio = make_loader(client.clone(), MediaKind::Audio, kind, state);

    // Handle the case when we want to play an item
    let play_item = move |item: MediaItem, kind: MediaKind| {
        playback_item.set(Some((item, kind)));
    };

    rsx! {
        section { class: "page",
            h2 { "Media library" }
            div { class: "tab-row",
                button { class: "{movies_class}", onclick: on_movies, "Movies" }
                button { class: "{shows_class}", onclick: on_shows, "TV Shows" }
                button { class: "{audio_class}", onclick: on_audio, "Audio" }
            }
            match playback_item() {
                Some((item, kind)) => rsx! {
                    PlaybackView { 
                        item: item, 
                        kind: kind,
                        on_back: move |_| playback_item.set(None) 
                    }
                },
                None => rsx! {
                    MediaListView { 
                        state: state(), 
                        kind: active,
                        on_play: play_item
                    }
                }
            }
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
fn MediaListView(state: LibraryState, kind: MediaKind, on_play: EventHandler<(MediaItem, MediaKind)>) -> Element {
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
                            MediaCard { 
                                key: "{item.id}", 
                                item: item, 
                                kind: kind,
                                on_play: on_play
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MediaCard(item: MediaItem, kind: MediaKind, on_play: EventHandler<(MediaItem, MediaKind)>) -> Element {
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
            button {
                class: "play-button",
                onclick: move |_| on_play.invoke((item.clone(), kind)),
                "▶ Play"
            }
        }
    }
}

#[component]
fn PlaybackView(item: MediaItem, kind: MediaKind, on_back: EventHandler<()>) -> Element {
    rsx! {
        section { class: "page",
            button { 
                class: "back-button",
                onclick: on_back,
                "← Back to Library"
            }
            h2 { "Playing: {item.title}" }
            div { class: "playback-container",
                div { class: "video-placeholder",
                    h3 { "Video/Audio Player" }
                    p { "Media Type: {kind.label()}" }
                    p { "Title: {item.title}" }
                    if let Some(year) = item.year {
                        p { "Year: {year}" }
                    }
                    if let Some(media_type) = &item.media_type {
                        p { "Format: {media_type}" }
                    }
                    p { "ID: {item.id}" }
                }
            }
        }
    }
}
