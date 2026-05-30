use crate::mk_lib_database;
use askama::Template;
use axum::{
    extract::{Path, Query},
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use serde::Deserialize;
use serde_json::Value;
use sqlx::postgres::PgPool;

const YOUTUBE_BASE_URL: &str = "https://www.youtube.com";
const YOUTUBE_VIDEO_ID_LENGTH: usize = 11;
const YOUTUBE_RESULTS_LIMIT: usize = 30;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Clone, Debug)]
struct YoutubeVideoItem {
    id: String,
    title: String,
    channel_title: String,
    duration: String,
    thumbnail_url: String,
}

#[derive(Template)]
#[template(path = "bss_user/internet/bss_user_internet_youtube.html")]
struct UserInternetYoutubeTemplate<'a> {
    template_data: &'a Vec<YoutubeVideoItem>,
    template_data_exists: &'a bool,
    search_query: &'a str,
    page_title: Option<String>,
}

#[derive(Deserialize)]
pub struct YoutubeSearchQuery {
    query: Option<String>,
}

pub async fn user_inter_youtube(
    method: Method,
    Query(query): Query<YoutubeSearchQuery>,
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
        let template = TemplateError401Context {};
        let reply_html = template.render().map_err(|e| e.to_string())?;
        return (StatusCode::UNAUTHORIZED, Html(reply_html).into_response());
    }

    let search_text = query.query.as_deref().unwrap_or("").trim().to_string();
    let youtube_videos = fetch_youtube_browser_results(&search_text)
        .await
        .unwrap_or_default();
    let has_data = !youtube_videos.is_empty();

    let template = UserInternetYoutubeTemplate {
        template_data: &youtube_videos,
        template_data_exists: &has_data,
        search_query: &search_text,
        page_title: Some("MediaKraken Youtube".to_string()),
    };

    let reply_html = template.render().map_err(|e| e.to_string())?;
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_user/internet/bss_user_internet_youtube_detail.html")]
struct UserInternetYoutubeDetailTemplate<'a> {
    template_youtube_video_guid: &'a str,
    template_embed_url: &'a str,
    page_title: Option<String>,
}

pub async fn user_inter_youtube_detail(
    method: Method,
    Path(guid): Path<String>,
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
        let template = TemplateError401Context {};
        let reply_html = template.render().map_err(|e| e.to_string())?;
        return (StatusCode::UNAUTHORIZED, Html(reply_html).into_response());
    }

    if !is_valid_video_id(&guid) {
        return (
            StatusCode::BAD_REQUEST,
            Html("Invalid YouTube video id".to_string()).into_response(),
        );
    }

    let embed_url = format!("https://www.youtube.com/embed/{}?autoplay=1", guid);
    let template = UserInternetYoutubeDetailTemplate {
        template_youtube_video_guid: guid.as_str(),
        template_embed_url: embed_url.as_str(),
        page_title: Some("MediaKraken Youtube Playback".to_string()),
    };
    let reply_html = template.render().map_err(|e| e.to_string())?;
    (StatusCode::OK, Html(reply_html).into_response())
}

async fn fetch_youtube_browser_results(
    search_text: &str,
) -> Result<Vec<YoutubeVideoItem>, reqwest::Error> {
    let mut request_url = format!("{}/feed/trending", YOUTUBE_BASE_URL);
    if !search_text.is_empty() {
        request_url = format!(
            "{}/results?search_query={}",
            YOUTUBE_BASE_URL,
            urlencoding::encode(search_text)
        );
    }

    let page_body = reqwest::Client::new()
        .get(request_url)
        .header("Accept-Language", "en-US,en;q=0.9")
        .send()
        .await?
        .text()
        .await?;

    Ok(extract_videos_from_initial_data(&page_body))
}

fn extract_videos_from_initial_data(page_body: &str) -> Vec<YoutubeVideoItem> {
    let Some(json_value) = find_json_block(page_body, "ytInitialData") else {
        return Vec::new();
    };

    let Ok(root_json): Result<Value, _> = serde_json::from_str(json_value) else {
        return Vec::new();
    };

    let mut results = Vec::new();
    collect_video_renderers(&root_json, &mut results);

    results
        .into_iter()
        .filter(|item| is_valid_video_id(&item.id))
        .take(YOUTUBE_RESULTS_LIMIT)
        .collect()
}

fn find_json_block<'a>(page_body: &'a str, marker: &str) -> Option<&'a str> {
    let marker_index = page_body.find(marker)?;
    let after_marker = &page_body[marker_index..];
    let start_brace_offset = after_marker.find('{')?;

    let mut in_string = false;
    let mut escaping = false;
    let mut depth = 0usize;
    let mut start_index = 0usize;

    for (offset, ch) in after_marker.char_indices().skip(start_brace_offset) {
        if in_string {
            if escaping {
                escaping = false;
                continue;
            }

            if ch == '\\' {
                escaping = true;
            } else if ch == '"' {
                in_string = false;
            }
            continue;
        }

        if ch == '"' {
            in_string = true;
            continue;
        }

        if ch == '{' {
            if depth == 0 {
                start_index = offset;
            }
            depth += 1;
            continue;
        }

        if ch == '}' {
            if depth == 0 {
                return None;
            }

            depth -= 1;
            if depth == 0 {
                let end_index = offset + ch.len_utf8();
                return after_marker.get(start_index..end_index);
            }
        }
    }

    None
}

fn collect_video_renderers(value: &Value, results: &mut Vec<YoutubeVideoItem>) {
    match value {
        Value::Object(map) => {
            if let Some(video_renderer) = map.get("videoRenderer") {
                if let Some(item) = parse_video_renderer(video_renderer) {
                    results.push(item);
                }
            }

            for nested in map.values() {
                collect_video_renderers(nested, results);
            }
        }
        Value::Array(array) => {
            for nested in array {
                collect_video_renderers(nested, results);
            }
        }
        _ => {}
    }
}

fn parse_video_renderer(video_renderer: &Value) -> Option<YoutubeVideoItem> {
    let id = video_renderer.get("videoId")?.as_str()?.to_string();

    let title = video_renderer
        .get("title")
        .and_then(extract_runs_text)
        .unwrap_or_else(|| "Untitled video".to_string());

    let channel_title = video_renderer
        .get("ownerText")
        .and_then(extract_runs_text)
        .unwrap_or_else(|| "Unknown channel".to_string());

    let duration = video_renderer
        .get("lengthText")
        .and_then(extract_runs_text)
        .unwrap_or_else(|| "Live".to_string());

    let thumbnail_url = video_renderer
        .get("thumbnail")
        .and_then(|thumb| thumb.get("thumbnails"))
        .and_then(Value::as_array)
        .and_then(|thumbs| thumbs.last())
        .and_then(|thumb| thumb.get("url"))
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();

    Some(YoutubeVideoItem {
        id,
        title,
        channel_title,
        duration,
        thumbnail_url,
    })
}

fn extract_runs_text(value: &Value) -> Option<String> {
    if let Some(simple_text) = value.get("simpleText").and_then(Value::as_str) {
        return Some(simple_text.to_string());
    }

    value
        .get("runs")
        .and_then(Value::as_array)
        .map(|runs| {
            runs.iter()
                .filter_map(|run| run.get("text").and_then(Value::as_str))
                .collect::<String>()
        })
        .filter(|text| !text.is_empty())
}

fn is_valid_video_id(video_id: &str) -> bool {
    video_id.len() == YOUTUBE_VIDEO_ID_LENGTH
        && video_id.chars().all(|character| {
            character.is_ascii_alphanumeric() || character == '-' || character == '_'
        })
}
