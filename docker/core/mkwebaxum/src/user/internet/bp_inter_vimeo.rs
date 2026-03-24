use crate::AppState;
use crate::mk_lib_database;
use askama::Template;
use axum::{
    extract::{Path, Query, State},
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use serde::Deserialize;
use sqlx::postgres::PgPool;
use std::collections::BTreeMap;

const VIMEO_BROWSE_PAGE_SIZE: usize = 24;
const VIMEO_CHANNEL_DEFAULT: &str = "staffpicks";

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_error/bss_error_500.html")]
struct TemplateError500Context {
    page_title: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VimeoBrowseQuery {
    #[serde(default)]
    q: String,
    #[serde(default = "default_page")]
    page: usize,
}

fn default_page() -> usize {
    1
}

#[derive(Debug, Deserialize)]
struct VimeoVideoItemRaw {
    id: u64,
    title: Option<String>,
    description: Option<String>,
    upload_date: Option<String>,
    duration: Option<u64>,
    thumbnail_large: Option<String>,
    url: Option<String>,
    user_name: Option<String>,
}

#[derive(Debug, Clone)]
struct VimeoVideoItem {
    id: String,
    title: String,
    description_preview: String,
    upload_date: String,
    duration: String,
    thumbnail_url: Option<String>,
    user_name: String,
    direct_url: Option<String>,
}

#[derive(Template)]
#[template(path = "bss_user/internet/bss_user_internet_vimeo.html")]
struct UserInternetVimeoTemplate {
    videos: Vec<VimeoVideoItem>,
    has_videos: bool,
    query: String,
    page: usize,
    pagination_prev: Option<usize>,
    pagination_next: Option<usize>,
    page_title: Option<String>,
}

pub async fn user_inter_vimeo(
    State(_state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Query(params): Query<VimeoBrowseQuery>,
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

    if params.page == 0 {
        return (
            StatusCode::BAD_REQUEST,
            Html("Invalid page. Page must be 1 or greater.".to_string()).into_response(),
        );
    }

    let fetched = fetch_vimeo_videos(params.page).await;
    let mut videos = match fetched {
        Ok(items) => items,
        Err(_) => return render_500(),
    };

    let query = params.q.trim().to_string();
    if !query.is_empty() {
        let query_lower = query.to_lowercase();
        videos.retain(|video| {
            video.title.to_lowercase().contains(&query_lower)
                || video
                    .description_preview
                    .to_lowercase()
                    .contains(&query_lower)
                || video.user_name.to_lowercase().contains(&query_lower)
        });
    }

    let has_videos = !videos.is_empty();
    let pagination_prev = if params.page > 1 {
        Some(params.page - 1)
    } else {
        None
    };
    let pagination_next = if has_videos {
        Some(params.page + 1)
    } else {
        None
    };

    let template = UserInternetVimeoTemplate {
        videos,
        has_videos,
        query,
        page: params.page,
        pagination_prev,
        pagination_next,
        page_title: Some("MediaKraken Vimeo".to_string()),
    };

    match template.render() {
        Ok(reply_html) => (StatusCode::OK, Html(reply_html).into_response()),
        Err(_) => render_500(),
    }
}

#[derive(Template)]
#[template(path = "bss_user/internet/bss_user_internet_vimeo_detail.html")]
struct UserInternetVimeoDetailTemplate {
    video_id: String,
    embed_url: String,
    watch_url: String,
    page_title: Option<String>,
}

pub async fn user_inter_vimeo_detail(
    State(_state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(video_id): Path<String>,
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

    if !video_id.chars().all(|ch| ch.is_ascii_digit()) {
        return (
            StatusCode::BAD_REQUEST,
            Html("Invalid Vimeo video id.".to_string()).into_response(),
        );
    }

    let template = UserInternetVimeoDetailTemplate {
        embed_url: format!("https://player.vimeo.com/video/{video_id}?autoplay=0"),
        watch_url: format!("https://vimeo.com/{video_id}"),
        video_id,
        page_title: Some("MediaKraken Vimeo Detail".to_string()),
    };

    match template.render() {
        Ok(reply_html) => (StatusCode::OK, Html(reply_html).into_response()),
        Err(_) => render_500(),
    }
}

async fn fetch_vimeo_videos(page: usize) -> Result<Vec<VimeoVideoItem>, reqwest::Error> {
    let mut query_params = BTreeMap::new();
    query_params.insert("page", page.to_string());
    query_params.insert("per_page", VIMEO_BROWSE_PAGE_SIZE.to_string());

    let url = format!("https://vimeo.com/api/v2/channel/{VIMEO_CHANNEL_DEFAULT}/videos.json");
    let response = reqwest::Client::new()
        .get(url)
        .query(&query_params)
        .send()
        .await?;
    let raw: Vec<VimeoVideoItemRaw> = response.error_for_status()?.json().await?;
    Ok(raw.into_iter().map(map_video_item).collect())
}

fn map_video_item(raw: VimeoVideoItemRaw) -> VimeoVideoItem {
    let title = raw
        .title
        .unwrap_or_else(|| "Untitled Vimeo video".to_string());
    let description_preview = truncate(raw.description.as_deref().unwrap_or(""), 180);
    let upload_date = raw
        .upload_date
        .unwrap_or_else(|| "Unknown upload date".to_string());
    let duration = format_duration(raw.duration.unwrap_or(0));
    let user_name = raw
        .user_name
        .unwrap_or_else(|| "Unknown creator".to_string());

    VimeoVideoItem {
        id: raw.id.to_string(),
        title,
        description_preview,
        upload_date,
        duration,
        thumbnail_url: raw.thumbnail_large,
        user_name,
        direct_url: raw.url,
    }
}

fn format_duration(total_seconds: u64) -> String {
    if total_seconds == 0 {
        return "Unknown".to_string();
    }
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    if hours > 0 {
        format!("{hours:02}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes:02}:{seconds:02}")
    }
}

fn truncate(input: &str, max_chars: usize) -> String {
    if input.chars().count() <= max_chars {
        return input.to_string();
    }
    let trimmed: String = input.chars().take(max_chars).collect();
    format!("{trimmed}…")
}

fn render_401() -> (StatusCode, axum::response::Response) {
    let template = TemplateError401Context {};
    match template.render() {
        Ok(reply_html) => (StatusCode::UNAUTHORIZED, Html(reply_html).into_response()),
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            Html(String::from("Unauthorized")).into_response(),
        ),
    }
}

fn render_500() -> (StatusCode, axum::response::Response) {
    let template = TemplateError500Context {
        page_title: Some("MediaKraken Error".to_string()),
    };
    match template.render() {
        Ok(reply_html) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(reply_html).into_response(),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Html(String::from("Internal server error")).into_response(),
        ),
    }
}
