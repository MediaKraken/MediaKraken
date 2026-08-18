use dioxus::prelude::*;

use crate::api::ApiClient;
use crate::models::{MediaItem, MediaKind};

#[derive(Clone, Debug, PartialEq)]
pub enum PlaybackState {
    Idle,
    Loading,
    Playing {
        item: MediaItem,
        media_type: String,
        stream_url: String,
    },
    Paused {
        item: MediaItem,
        media_type: String,
        stream_url: String,
        position: f64, // in seconds
    },
    Error(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum PlaybackMode {
    Movie,
    Show,
    Audio,
}

#[component]
pub fn PlaybackPage(client: ApiClient, media_item: MediaItem, kind: MediaKind) -> Element {
    let mut playback_state = use_signal(|| PlaybackState::Idle);
    let mut volume = use_signal(|| 80u8);
    let mut is_muted = use_signal(|| false);
    let mut is_paused = use_signal(|| false);
    let mut progress = use_signal(|| 0f64);

    // Fetch stream URL when page loads
    use_effect(move || {
        let media_item = media_item.clone();
        spawn(async move {
            let stream_url = match kind {
                MediaKind::Movie => format!("/api/v1/media/movies/{}/stream", media_item.id),
                MediaKind::Show => format!("/api/v1/media/shows/{}/stream", media_item.id),
                MediaKind::Audio => format!("/api/v1/media/audio/{}/stream", media_item.id),
            };
            let media_type = match kind {
                MediaKind::Movie => "video/mp4".to_string(),
                MediaKind::Show => "video/mp4".to_string(),
                MediaKind::Audio => "audio/mp3".to_string(),
            };
            playback_state.set(PlaybackState::Playing {
                item: media_item,
                media_type,
                stream_url,
            });
        });
    });
    rsx! {
        section { class: "page",
            h2 { "Media Playback" }
            match playback_state() {
                PlaybackState::Idle => rsx! {
                    p { class: "muted", "Loading playback..." }
                },
                PlaybackState::Loading => rsx! {
                    p { class: "muted", "Loading stream..." }
                },
                PlaybackState::Playing { item, media_type, stream_url } => rsx! {
                    div { class: "playback-container",
                        VideoPlayer {
                            stream_url: stream_url.clone(),
                            media_type: media_type.clone(),
                            volume: volume(),
                            is_muted: is_muted(),
                            is_paused: is_paused(),
                            progress: move |new_progress| progress.set(new_progress),
                        },
                        MediaInfo { item: item.clone() },
                        PlaybackControls {
                            is_paused: is_paused(),
                            is_muted: is_muted(),
                            on_play_pause: Callback::new(move |_| {
                                is_paused.set(!is_paused());
                            }),
                            on_volume_change: Callback::new(move |new_volume| {
                                volume.set(new_volume);
                            }),
                            on_mute_toggle: Callback::new(move |_| {
                                is_muted.set(!is_muted());
                            }),
                            on_stop: Callback::new(move |_| {
                                playback_state.set(PlaybackState::Idle);
                            }),
                        },
                    }
                },
                PlaybackState::Paused { item, media_type, stream_url, .. } => rsx! {
                    div { class: "playback-container",
                        VideoPlayer {
                            stream_url: stream_url.clone(),
                            media_type: media_type.clone(),
                            volume: volume(),
                            is_muted: is_muted(),
                            is_paused: is_paused(),
                            progress: move |new_progress| progress.set(new_progress),
                        },
                        MediaInfo { item: item.clone() },
                        PlaybackControls {
                            is_paused: is_paused(),
                            is_muted: is_muted(),
                            on_play_pause: Callback::new(move |_| {
                                is_paused.set(!is_paused());
                            }),
                            on_volume_change: Callback::new(move |new_volume| {
                                volume.set(new_volume);
                            }),
                            on_mute_toggle: Callback::new(move |_| {
                                is_muted.set(!is_muted());
                            }),
                            on_stop: Callback::new(move |_| {
                                playback_state.set(PlaybackState::Idle);
                            }),
                        }
                    }
                },
                PlaybackState::Error(message) => rsx! {
                    p { class: "error", "Playback Error: {message}" }
                }
            }
        }
    }
}

#[component]
fn VideoPlayer(props: VideoPlayerProps) -> Element {
    let VideoPlayerProps {
        stream_url,
        media_type,
        volume,
        is_muted,
        is_paused,
        ..
    } = props;

    // We'll simulate the video player with a placeholder
    rsx! {
        div { class: "video-player",
            div { class: "video-placeholder",
                h3 { "Video Player" }
                p { "Stream URL: {stream_url}" }
                p { "Media Type: {media_type}" }
                p { "Volume: {volume}" }
                p { "Muted: {is_muted}" }
                p { "Paused: {is_paused}" }
                // In a real implementation, this would be an actual HTML5 video element
                // or a native media player component for mobile
            }
        }
    }
}

#[derive(Clone, Props, PartialEq)]
struct VideoPlayerProps {
    stream_url: String,
    media_type: String,
    volume: u8,
    is_muted: bool,
    is_paused: bool,
    progress: Callback<f64>,
}

#[component]
fn MediaInfo(item: MediaItem) -> Element {
    let year = item
        .year
        .map(|y| y.to_string())
        .unwrap_or_else(|| "—".to_string());
    let media_type = item.media_type.clone().unwrap_or_default();
    let overview = item.overview.clone().unwrap_or_default();

    rsx! {
        div { class: "media-info",
            h2 { "{item.title}" }
            p { "{media_type} · {year}" }
            if !overview.is_empty() {
                p { "{overview}" }
            }
        }
    }
}

#[component]
fn PlaybackControls(props: PlaybackControlsProps) -> Element {
    let PlaybackControlsProps {
        is_paused,
        is_muted,
        on_play_pause,
        on_volume_change,
        on_mute_toggle,
        on_stop,
    } = props;

    rsx! {
        div { class: "playback-controls",
            button {
                class: "control-btn",
                onclick: move |_| on_play_pause(()),
                if is_paused {
                    "▶ Play"
                } else {
                    "⏸ Pause"
                }
            }
            button {
                class: "control-btn",
                onclick: move |_| on_stop(()),
                "⏹ Stop"
            }
            button {
                class: "control-btn",
                onclick: move |_| on_mute_toggle(()),
                if is_muted {
                    "🔈 Unmute"
                } else {
                    "🔇 Mute"
                }
            }
            input {
                class: "volume-slider",
                r#type: "range",
                min: "0",
                max: "100",
                value: "80",
                oninput: move |event| {
                    let volume = event.value().parse::<u8>().unwrap_or(80);
                    on_volume_change(volume);
                }
            }
        }
    }
}

#[derive(Clone, Props, PartialEq)]
struct PlaybackControlsProps {
    is_paused: bool,
    is_muted: bool,
    on_play_pause: Callback<()>,
    on_volume_change: Callback<u8>,
    on_mute_toggle: Callback<()>,
    on_stop: Callback<()>,
}
