use crate::AppState;
use crate::mk_lib_database;
use askama::Template;
use axum::{
    extract::{Path, State},
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use serde::Deserialize;
use sqlx::postgres::PgPool;
use std::collections::HashSet;
use std::env;

const TWITCH_TOP_STREAMS_URL: &str = "https://api.twitch.tv/helix/streams?first=24";
const TWITCH_DIRECTORY_URL: &str = "https://www.twitch.tv/directory/all";
const TWITCH_EMBED_PARENT_DEFAULT: &str = "localhost";

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_user/internet/bss_user_internet_twitch.html")]
struct UserInternetTwitchTVTemplate {
    streams: Vec<TwitchStreamBrowseItem>,
    has_streams: bool,
    error_message: Option<String>,
    page_title: Option<String>,
}

pub async fn user_inter_twitchtv(
    State(_state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::GET],
        false,
    )
    .requires(Rights::any([Rights::permission("User::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        return render_401();
    }

    let client_id = env::var("TWITCH_CLIENT_ID").ok();
    let bearer_token = env::var("TWITCH_APP_ACCESS_TOKEN").ok();

    let (streams, error_message) = match (client_id, bearer_token) {
        (Some(client_id), Some(bearer_token)) => {
            match fetch_twitch_streams(&client_id, &bearer_token).await {
                Ok(streams) => (streams, None),
                Err(()) => (
                    Vec::new(),
                    Some("Unable to load Twitch streams right now.".to_string()),
                ),
            }
        }
        _ => match fetch_twitch_directory_streams().await {
            Ok(streams) => (streams, None),
            Err(()) => (
                Vec::new(),
                Some("Unable to load Twitch streams right now.".to_string()),
            ),
        },
    };

    let template = UserInternetTwitchTVTemplate {
        has_streams: !streams.is_empty(),
        streams,
        error_message,
        page_title: Some("MediaKraken TwitchTV".to_string()),
    };

    match template.render() {
        Ok(reply_html) => (StatusCode::OK, Html(reply_html).into_response()),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html("Internal server error".to_string()).into_response(),
        ),
    }
}

#[derive(Template)]
#[template(path = "bss_user/internet/bss_user_internet_twitch_detail.html")]
struct UserInternetTwitchTVDetailTemplate {
    twitchtv_channel: String,
    twitch_embed_parent: String,
    page_title: Option<String>,
}

#[derive(Deserialize)]
struct TwitchStreamsResponse {
    data: Vec<TwitchStreamApiItem>,
}

#[derive(Deserialize)]
struct TwitchStreamApiItem {
    user_login: String,
    user_name: String,
    title: String,
    game_name: String,
    viewer_count: i64,
    thumbnail_url: String,
}

struct TwitchStreamBrowseItem {
    channel_login: String,
    channel_display_name: String,
    stream_title: String,
    game_name: String,
    viewer_count: i64,
    preview_image_url: String,
}

pub async fn user_inter_twitchtv_detail(
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(stream_name): Path<String>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::GET],
        false,
    )
    .requires(Rights::any([Rights::permission("User::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        return render_401();
    }

    if !is_valid_twitch_channel(&stream_name) {
        return (
            StatusCode::BAD_REQUEST,
            Html("Invalid channel".to_string()).into_response(),
        );
    }

    let template = UserInternetTwitchTVDetailTemplate {
        twitchtv_channel: stream_name,
        twitch_embed_parent: env::var("TWITCH_EMBED_PARENT")
            .unwrap_or_else(|_| TWITCH_EMBED_PARENT_DEFAULT.to_string()),
        page_title: Some("MediaKraken TwitchTV Stream".to_string()),
    };

    match template.render() {
        Ok(reply_html) => (StatusCode::OK, Html(reply_html).into_response()),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html("Internal server error".to_string()).into_response(),
        ),
    }
}

fn render_401() -> (StatusCode, axum::response::Response) {
    let template = TemplateError401Context {};
    match template.render() {
        Ok(reply_html) => (StatusCode::UNAUTHORIZED, Html(reply_html).into_response()),
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            Html("Unauthorized".to_string()).into_response(),
        ),
    }
}

async fn fetch_twitch_streams(
    client_id: &str,
    bearer_token: &str,
) -> Result<Vec<TwitchStreamBrowseItem>, ()> {
    let http_client = reqwest::Client::new();
    let response = http_client
        .get(TWITCH_TOP_STREAMS_URL)
        .header("Client-Id", client_id)
        .header("Authorization", format!("Bearer {bearer_token}"))
        .send()
        .await
        .map_err(|_| ())?;

    if !response.status().is_success() {
        return Err(());
    }

    let response_data: TwitchStreamsResponse = response.json().await.map_err(|_| ())?;

    Ok(response_data
        .data
        .into_iter()
        .map(|stream| TwitchStreamBrowseItem {
            channel_login: stream.user_login,
            channel_display_name: stream.user_name,
            stream_title: stream.title,
            game_name: if stream.game_name.is_empty() {
                "Not Available".to_string()
            } else {
                stream.game_name
            },
            viewer_count: stream.viewer_count,
            preview_image_url: stream
                .thumbnail_url
                .replace("{width}", "640")
                .replace("{height}", "360"),
        })
        .collect())
}

async fn fetch_twitch_directory_streams() -> Result<Vec<TwitchStreamBrowseItem>, ()> {
    let response = reqwest::Client::new()
        .get(TWITCH_DIRECTORY_URL)
        .header("User-Agent", "Mozilla/5.0 (MediaKraken)")
        .send()
        .await
        .map_err(|_| ())?;

    if !response.status().is_success() {
        return Err(());
    }

    let response_body = response.text().await.map_err(|_| ())?;
    let channels = extract_channels_from_directory_html(&response_body, 24);
    if channels.is_empty() {
        return Err(());
    }

    Ok(channels
        .into_iter()
        .map(|channel| TwitchStreamBrowseItem {
            channel_login: channel.clone(),
            channel_display_name: channel.clone(),
            stream_title: "Open channel".to_string(),
            game_name: "Browse".to_string(),
            viewer_count: 0,
            preview_image_url: format!(
                "https://static-cdn.jtvnw.net/previews-ttv/live_user_{}-640x360.jpg",
                channel
            ),
        })
        .collect())
}

fn extract_channels_from_directory_html(html: &str, limit: usize) -> Vec<String> {
    let mut channels = Vec::new();
    let mut seen = HashSet::new();
    let blocked_paths = [
        "directory",
        "downloads",
        "jobs",
        "login",
        "messages",
        "p",
        "prime",
        "search",
        "settings",
        "signup",
        "store",
        "subscriptions",
        "turbo",
        "videos",
        "wallet",
    ];

    let mut cursor = 0usize;
    while let Some(index) = html[cursor..].find("href=\"/") {
        let start = cursor + index + "href=\"/".len();
        let remainder = &html[start..];
        let Some(end) = remainder.find('"') else {
            break;
        };

        let candidate = &remainder[..end];
        cursor = start + end;

        if candidate.contains('/') || blocked_paths.contains(&candidate) {
            continue;
        }

        if !is_valid_twitch_channel(candidate) || !seen.insert(candidate.to_string()) {
            continue;
        }

        channels.push(candidate.to_string());
        if channels.len() >= limit {
            break;
        }
    }

    channels
}

fn is_valid_twitch_channel(channel: &str) -> bool {
    if channel.len() < 4 || channel.len() > 25 {
        return false;
    }

    channel
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}
