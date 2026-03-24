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
use sqlx::postgres::PgPool;

const FLICKR_PUBLIC_FEED_URL: &str = "https://www.flickr.com/services/feeds/photos_public.gne";
const MAX_TAG_LENGTH: usize = 120;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_error/bss_error_500.html")]
struct TemplateError500Context {
    page_title: Option<String>,
}

#[derive(Clone)]
struct FlickrBrowseItem {
    id: String,
    title: String,
    author: String,
    image_url: String,
    published: String,
    link_url: String,
    tags: String,
    description: String,
}

#[derive(Clone)]
struct FlickrDetailItem {
    id: String,
    title: String,
    author: String,
    image_url: String,
    published: String,
    link_url: String,
    tags: String,
    description: String,
}

#[derive(Template)]
#[template(path = "bss_user/internet/bss_user_internet_flickr.html")]
struct TemplateUserInternetFlickr {
    items: Vec<FlickrBrowseItem>,
    has_items: bool,
    tags: String,
    page_title: Option<String>,
}

#[derive(Template)]
#[template(path = "bss_user/internet/bss_user_internet_flickr_detail.html")]
struct TemplateUserInternetFlickrDetail {
    item: FlickrDetailItem,
    page_title: Option<String>,
}

#[derive(Deserialize)]
pub struct FlickrBrowseQuery {
    tags: Option<String>,
}

#[derive(Deserialize)]
pub struct FlickrDetailQuery {
    title: Option<String>,
    author: Option<String>,
    image: Option<String>,
    published: Option<String>,
    link: Option<String>,
    tags: Option<String>,
    description: Option<String>,
}

#[derive(Deserialize)]
struct FlickrFeedResponse {
    items: Vec<FlickrFeedItem>,
}

#[derive(Deserialize)]
struct FlickrFeedItem {
    title: String,
    link: String,
    media: FlickrMedia,
    #[serde(rename = "date_taken")]
    _date_taken: String,
    description: String,
    published: String,
    author: String,
    tags: String,
}

#[derive(Deserialize)]
struct FlickrMedia {
    m: String,
}

pub async fn user_inter_flickr(
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Query(query): Query<FlickrBrowseQuery>,
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

    let tags = sanitize_tags(query.tags.as_deref().unwrap_or_default());

    let feed =
        match fetch_flickr_public_feed(if tags.is_empty() { None } else { Some(&tags) }).await {
            Ok(items) => items,
            Err(_) => return render_500(),
        };

    let items = feed
        .into_iter()
        .filter_map(|item| map_feed_item_to_browse_item(&item))
        .collect::<Vec<FlickrBrowseItem>>();

    let template = TemplateUserInternetFlickr {
        has_items: !items.is_empty(),
        items,
        tags,
        page_title: Some("MediaKraken Flickr".to_string()),
    };

    match template.render() {
        Ok(reply_html) => (StatusCode::OK, Html(reply_html).into_response()),
        Err(_) => render_500(),
    }
}

pub async fn user_inter_flickr_detail(
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(guid): Path<String>,
    Query(query): Query<FlickrDetailQuery>,
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

    let from_query = build_detail_from_query(&guid, &query);
    let detail_item = if let Some(detail) = from_query {
        detail
    } else {
        let feed = match fetch_flickr_public_feed(None).await {
            Ok(items) => items,
            Err(_) => return render_500(),
        };

        match feed
            .iter()
            .find_map(|item| map_feed_item_to_detail_item(item, &guid))
        {
            Some(item) => item,
            None => {
                return (
                    StatusCode::NOT_FOUND,
                    Html(String::from("Flickr item not found")).into_response(),
                );
            }
        }
    };

    let template = TemplateUserInternetFlickrDetail {
        item: detail_item,
        page_title: Some("MediaKraken Flickr Detail".to_string()),
    };

    match template.render() {
        Ok(reply_html) => (StatusCode::OK, Html(reply_html).into_response()),
        Err(_) => render_500(),
    }
}

async fn fetch_flickr_public_feed(
    tags: Option<&str>,
) -> Result<Vec<FlickrFeedItem>, Box<dyn std::error::Error + Send + Sync>> {
    let mut request = reqwest::Client::new()
        .get(FLICKR_PUBLIC_FEED_URL)
        .query(&[("format", "json"), ("nojsoncallback", "1")]);

    if let Some(tag_filter) = tags {
        if !tag_filter.trim().is_empty() {
            request = request.query(&[("tags", tag_filter)]);
        }
    }

    let response = request.send().await?.error_for_status()?;
    let payload = response.json::<FlickrFeedResponse>().await?;
    Ok(payload.items)
}

fn map_feed_item_to_browse_item(item: &FlickrFeedItem) -> Option<FlickrBrowseItem> {
    let id = extract_photo_id(&item.link)?;

    Some(FlickrBrowseItem {
        id,
        title: fallback_text(&item.title, "Untitled"),
        author: fallback_text(&item.author, "Unknown author"),
        image_url: item.media.m.clone(),
        published: item.published.clone(),
        link_url: item.link.clone(),
        tags: fallback_text(&item.tags, "No tags"),
        description: fallback_text(&item.description, "No description."),
    })
}

fn map_feed_item_to_detail_item(
    item: &FlickrFeedItem,
    requested_id: &str,
) -> Option<FlickrDetailItem> {
    let id = extract_photo_id(&item.link)?;
    if id != requested_id {
        return None;
    }

    Some(FlickrDetailItem {
        id,
        title: fallback_text(&item.title, "Untitled"),
        author: fallback_text(&item.author, "Unknown author"),
        image_url: item.media.m.clone(),
        published: item.published.clone(),
        link_url: item.link.clone(),
        tags: fallback_text(&item.tags, "No tags"),
        description: fallback_text(&item.description, "No description."),
    })
}

fn build_detail_from_query(guid: &str, query: &FlickrDetailQuery) -> Option<FlickrDetailItem> {
    let (Some(image_url), Some(link_url)) = (query.image.clone(), query.link.clone()) else {
        return None;
    };

    Some(FlickrDetailItem {
        id: guid.to_string(),
        title: fallback_text(query.title.as_deref().unwrap_or_default(), "Untitled"),
        author: fallback_text(
            query.author.as_deref().unwrap_or_default(),
            "Unknown author",
        ),
        image_url,
        published: fallback_text(
            query.published.as_deref().unwrap_or_default(),
            "Unknown publish date",
        ),
        link_url,
        tags: fallback_text(query.tags.as_deref().unwrap_or_default(), "No tags"),
        description: fallback_text(
            query.description.as_deref().unwrap_or_default(),
            "No description.",
        ),
    })
}

fn extract_photo_id(link: &str) -> Option<String> {
    let path_without_query = link.split('?').next().unwrap_or_default();
    let trimmed = path_without_query.trim_end_matches('/');
    trimmed.rsplit('/').next().map(str::to_string)
}

fn fallback_text(input: &str, fallback: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        fallback.to_string()
    } else {
        trimmed.to_string()
    }
}

fn sanitize_tags(input: &str) -> String {
    let compact = input
        .split(',')
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<&str>>()
        .join(",");

    if compact.len() > MAX_TAG_LENGTH {
        compact[..MAX_TAG_LENGTH].to_string()
    } else {
        compact
    }
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
