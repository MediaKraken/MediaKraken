use crate::mk_lib_database;
use crate::AppState;
use askama::Template;
use axum::{
    extract::{Path, State},
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use serde_json::Value;
use sqlx::{postgres::PgPool, FromRow};

const PAGE_SIZE: i64 = 24;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_error/bss_error_500.html")]
struct TemplateError500Context {
    page_title: Option<String>,
}

#[derive(FromRow)]
struct OpenLibraryWorkRow {
    mm_openlib_work_id: String,
    mm_openlib_work_json: Value,
}

struct OpenLibraryBrowseItem {
    work_id: String,
    route_work_id: String,
    title: String,
    author_name: String,
    first_publish_year: String,
    edition_count: String,
    subject_preview: Vec<String>,
    subject_more_count: usize,
    cover_url: Option<String>,
}

struct OpenLibraryDetailItem {
    work_id: String,
    title: String,
    description: String,
    authors: Vec<String>,
    subjects: Vec<String>,
    first_publish_year: String,
    cover_url: Option<String>,
}

#[derive(Template)]
#[template(path = "bss_user/internet/bss_user_internet_openlibrary.html")]
struct TemplateOpenLibraryBrowse {
    items: Vec<OpenLibraryBrowseItem>,
    has_items: bool,
    page: i64,
    pagination_prev: Option<i64>,
    pagination_next: Option<i64>,
    page_title: Option<String>,
}

#[derive(Template)]
#[template(path = "bss_user/internet/bss_user_internet_openlibrary_detail.html")]
struct TemplateOpenLibraryDetail {
    item: OpenLibraryDetailItem,
    page_title: Option<String>,
}

pub async fn user_inter_openlibrary(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(page): Path<i64>,
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

    if page < 1 {
        return (
            StatusCode::BAD_REQUEST,
            Html(String::from("Invalid page")).into_response(),
        );
    }

    let db_offset = (page - 1) * PAGE_SIZE;
    let rows_result = sqlx::query_as::<_, OpenLibraryWorkRow>(
        r#"select mm_openlib_work_id, mm_openlib_work_json
           from mm_openlib_work
           order by mm_openlib_work_id
           offset $1 limit $2"#,
    )
    .bind(db_offset)
    .bind(PAGE_SIZE)
    .fetch_all(&state.sqlx_pool_ro)
    .await;

    let count_result = sqlx::query_scalar::<_, i64>("select count(*) from mm_openlib_work")
        .fetch_one(&state.sqlx_pool_ro)
        .await;

    let (rows, total_count) = match (rows_result, count_result) {
        (Ok(rows), Ok(total_count)) => (rows, total_count),
        _ => return render_500(),
    };

    let items: Vec<OpenLibraryBrowseItem> = rows.iter().map(map_work_to_browse_item).collect();
    let has_items = !items.is_empty();
    let has_prev = page > 1;
    let has_next = db_offset + PAGE_SIZE < total_count;

    let template = TemplateOpenLibraryBrowse {
        items,
        has_items,
        page,
        pagination_prev: if has_prev { Some(page - 1) } else { None },
        pagination_next: if has_next { Some(page + 1) } else { None },
        page_title: Some("MediaKraken OpenLibrary Browse".to_string()),
    };

    match template.render() {
        Ok(reply_html) => (StatusCode::OK, Html(reply_html).into_response()),
        Err(_) => render_500(),
    }
}

pub async fn user_inter_openlibrary_detail(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(work_id): Path<String>,
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

    let Some(canonical_work_id) = canonicalize_work_id(&work_id) else {
        return (
            StatusCode::BAD_REQUEST,
            Html(String::from("Invalid work id")).into_response(),
        );
    };

    let row_result = sqlx::query_as::<_, OpenLibraryWorkRow>(
        r#"select mm_openlib_work_id, mm_openlib_work_json
           from mm_openlib_work
           where mm_openlib_work_id = $1"#,
    )
    .bind(&canonical_work_id)
    .fetch_optional(&state.sqlx_pool_ro)
    .await;

    let row = match row_result {
        Ok(Some(row)) => row,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Html(String::from("Not found")).into_response(),
            )
        }
        Err(_) => return render_500(),
    };

    let template = TemplateOpenLibraryDetail {
        item: map_work_to_detail_item(&row),
        page_title: Some("MediaKraken OpenLibrary Detail".to_string()),
    };

    match template.render() {
        Ok(reply_html) => (StatusCode::OK, Html(reply_html).into_response()),
        Err(_) => render_500(),
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

fn map_work_to_browse_item(row: &OpenLibraryWorkRow) -> OpenLibraryBrowseItem {
    let title =
        read_string(&row.mm_openlib_work_json, "title").unwrap_or_else(|| "Untitled".to_string());
    let author_name = read_first_string(&row.mm_openlib_work_json, "author_name")
        .unwrap_or_else(|| "Unknown author".to_string());
    let first_publish_year = read_i64(&row.mm_openlib_work_json, "first_publish_year")
        .map_or_else(|| "Unknown".to_string(), |year| year.to_string());
    let edition_count = read_i64(&row.mm_openlib_work_json, "edition_count")
        .map_or_else(|| "0".to_string(), |count| count.to_string());
    let all_subjects = read_string_array(&row.mm_openlib_work_json, "subject");
    let subject_preview: Vec<String> = all_subjects.iter().take(3).cloned().collect();
    let subject_more_count = all_subjects.len().saturating_sub(subject_preview.len());
    let cover_url = read_i64(&row.mm_openlib_work_json, "cover_i")
        .map(|cover| format!("https://covers.openlibrary.org/b/id/{cover}-M.jpg"));

    let route_work_id = row.mm_openlib_work_id.trim_start_matches('/').to_string();

    OpenLibraryBrowseItem {
        work_id: row.mm_openlib_work_id.clone(),
        route_work_id,
        title,
        author_name,
        first_publish_year,
        edition_count,
        subject_preview,
        subject_more_count,
        cover_url,
    }
}

fn map_work_to_detail_item(row: &OpenLibraryWorkRow) -> OpenLibraryDetailItem {
    let title =
        read_string(&row.mm_openlib_work_json, "title").unwrap_or_else(|| "Untitled".to_string());
    let authors = read_string_array(&row.mm_openlib_work_json, "author_name");
    let subjects = read_string_array(&row.mm_openlib_work_json, "subject");
    let description = read_description(&row.mm_openlib_work_json)
        .unwrap_or_else(|| "No description available for this work.".to_string());
    let first_publish_year = read_i64(&row.mm_openlib_work_json, "first_publish_year")
        .map_or_else(|| "Unknown".to_string(), |year| year.to_string());
    let cover_url = read_i64(&row.mm_openlib_work_json, "cover_i")
        .map(|cover| format!("https://covers.openlibrary.org/b/id/{cover}-L.jpg"));

    OpenLibraryDetailItem {
        work_id: row.mm_openlib_work_id.clone(),
        title,
        description,
        authors,
        subjects,
        first_publish_year,
        cover_url,
    }
}

fn read_description(value: &Value) -> Option<String> {
    match value.get("description") {
        Some(Value::String(text)) => Some(text.clone()),
        Some(Value::Object(map)) => map
            .get("value")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned),
        _ => None,
    }
}

fn read_string(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
}

fn read_first_string(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(Value::as_array)
        .and_then(|arr| arr.first())
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
}

fn read_i64(value: &Value, key: &str) -> Option<i64> {
    value.get(key).and_then(Value::as_i64)
}

fn read_string_array(value: &Value, key: &str) -> Vec<String> {
    value
        .get(key)
        .and_then(Value::as_array)
        .map(|array| {
            array
                .iter()
                .filter_map(Value::as_str)
                .map(ToOwned::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

fn canonicalize_work_id(work_id: &str) -> Option<String> {
    let trimmed = work_id.trim();
    if trimmed.is_empty() {
        return None;
    }

    let canonical = if trimmed.starts_with('/') {
        trimmed.to_string()
    } else {
        format!("/{trimmed}")
    };

    if canonical.len() > 255 {
        return None;
    }

    Some(canonical)
}
