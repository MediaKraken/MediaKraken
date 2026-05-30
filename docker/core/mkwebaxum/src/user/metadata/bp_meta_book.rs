use crate::AppState;
use crate::mk_lib_database;
use crate::user_preferences;
use askama::Template;
use axum::extract::Query;
use axum::extract::State;
use axum::response::Redirect;
use axum::{
    Extension,
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session::{SessionConfig, SessionLayer};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use mk_lib_common::mk_lib_common_pagination;
use serde::Deserialize;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_book.html")]
struct TemplateMetaBookContext<'a> {
    template_data:
        &'a Vec<mk_lib_database::database_metadata::mk_lib_database_metadata_book::DBMetaBookList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
    page_title: Option<String>,
    pub current: Option<String>,
    pub genre_filter: Option<String>,
    pub genre_filter_query: Option<String>,
    pub primary_language_filter: Option<String>,
    pub status_filter: Option<String>,
    pub base_path: String,
}

#[derive(Debug, Deserialize)]
pub struct FilterQuery {
    pub starts_with: Option<String>,
}

fn normalize_starts_with(raw: Option<&str>) -> Option<String> {
    let value = raw.map(str::trim).filter(|value| !value.is_empty())?;
    let first_char = value.chars().next()?;

    if first_char == '#' {
        return Some("#".to_string());
    }

    if first_char.is_ascii_alphanumeric() {
        return Some(first_char.to_ascii_uppercase().to_string());
    }

    Some("#".to_string())
}

pub async fn user_metadata_book(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(page): Path<i64>,
    Query(params): Query<FilterQuery>,
) -> impl IntoResponse {
    let starts_with = normalize_starts_with(params.starts_with.as_deref());
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
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let pagination_count =
            user_preferences::load_user_pagination_count(&state.sqlx_pool_ro, current_user.id)
                .await
                .unwrap_or(user_preferences::DEFAULT_PAGINATION_COUNT);
        let db_offset: i64 = (page * pagination_count) - pagination_count;
        let total_pages: i64 =
        mk_lib_database::database_metadata::mk_lib_database_metadata_book::mk_lib_database_metadata_book_count(
            &state.sqlx_pool_ro,
            starts_with.clone().unwrap_or_default(),
        )
        .await
        ?;
        let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
            total_pages,
            page,
            "/user/metadata/book".to_string(),
            starts_with.as_deref(),
            pagination_count,
        )
        .await
        ?;
        let book_list =
        mk_lib_database::database_metadata::mk_lib_database_metadata_book::mk_lib_database_metadata_book_read(
           &state.sqlx_pool_ro,
            starts_with.clone().unwrap_or_default(),
            db_offset,
            pagination_count,
        )
        .await
        ?;
        let mut template_data_exists = false;
        if book_list.len() > 0 {
            template_data_exists = true;
        }
        let page_usize = page as usize;
        let template = TemplateMetaBookContext {
            template_data: &book_list,
            template_data_exists: &template_data_exists,
            pagination_bar: &pagination_html,
            page: &page_usize,
            page_title: Some("MediaKraken Metadata Books".to_string()),
            current: starts_with,
            genre_filter: None,
            genre_filter_query: None,
            primary_language_filter: None,
            status_filter: None,
            base_path: "/user/metadata/book".to_string(),
        };
        let reply_html = template.render().map_err(|e| e.to_string())?;
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_book_detail.html")]
struct TemplateMetaBookDetailContext {
    template_data: serde_json::Value,
    page_title: Option<String>,
}

pub async fn user_metadata_book_detail(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(guid): Path<uuid::Uuid>,
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
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let detail_data =
        mk_lib_database::database_metadata::mk_lib_database_metadata_book::mk_lib_database_metadata_book_detail(
            &state.sqlx_pool_ro, guid,
        )
        .await
        ?;
        let template = TemplateMetaBookDetailContext {
            template_data: detail_data,
            page_title: Some("MediaKraken Metadata Book Detail".to_string()),
        };
        let reply_html = template.render().map_err(|e| e.to_string())?;
        (StatusCode::OK, Html(reply_html).into_response())
    }
}
