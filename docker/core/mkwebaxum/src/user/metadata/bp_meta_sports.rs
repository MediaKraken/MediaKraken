use crate::AppState;
use crate::mk_lib_database;
use crate::user_preferences;
use askama::Template;
use axum::{
    extract::{Path, Query, State},
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use mk_lib_common::mk_lib_common_pagination;
use serde::Deserialize;
use serde_json::json;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_error/bss_error_500.html")]
struct TemplateError500Context {}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_sports.html")]
struct TemplateMetaSportsContext<'a> {
    template_data: &'a Vec<
        mk_lib_database::database_metadata::mk_lib_database_metadata_sports::DBMetaSportsList,
    >,
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

pub async fn user_metadata_sports(
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
        let reply_html = template.render().unwrap();
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let pagination_count =
            user_preferences::load_user_pagination_count(&state.sqlx_pool_ro, current_user.id)
                .await
                .unwrap_or(user_preferences::DEFAULT_PAGINATION_COUNT);
        let db_offset: i64 = (page * pagination_count) - pagination_count;
        let total_pages = match mk_lib_database::database_metadata::mk_lib_database_metadata_sports::mk_lib_database_metadata_sports_count(
            &state.sqlx_pool_ro,
            starts_with.clone().unwrap_or_default(),
        )
        .await
        {
            Ok(total_pages) => total_pages,
            Err(err) => {
                eprintln!("sports metadata count failed: {err}");
                let template = TemplateError500Context {};
                let reply_html = template.render().unwrap_or_default();
                return (StatusCode::INTERNAL_SERVER_ERROR, Html(reply_html).into_response());
            }
        };
        let pagination_html = match mk_lib_common_pagination::mk_lib_common_paginate(
            total_pages,
            page,
            "/user/metadata/sports".to_string(),
            starts_with.as_deref(),
            pagination_count,
        )
        .await
        {
            Ok(pagination_html) => pagination_html,
            Err(err) => {
                eprintln!("sports metadata pagination failed: {err}");
                let template = TemplateError500Context {};
                let reply_html = template.render().unwrap_or_default();
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(reply_html).into_response(),
                );
            }
        };
        let sports_list = match mk_lib_database::database_metadata::mk_lib_database_metadata_sports::mk_lib_database_metadata_sports_read(
            &state.sqlx_pool_ro,
            starts_with.clone().unwrap_or_default(),
            db_offset,
            pagination_count,
        )
        .await
        {
            Ok(sports_list) => sports_list,
            Err(err) => {
                eprintln!("sports metadata read failed: {err}");
                let template = TemplateError500Context {};
                let reply_html = template.render().unwrap_or_default();
                return (StatusCode::INTERNAL_SERVER_ERROR, Html(reply_html).into_response());
            }
        };
        let template_data_exists = !sports_list.is_empty();
        let page_usize = page as usize;
        let template = TemplateMetaSportsContext {
            template_data: &sports_list,
            template_data_exists: &template_data_exists,
            pagination_bar: &pagination_html,
            page: &page_usize,
            page_title: Some("MediaKraken Metadata Sports".to_string()),
            current: starts_with,
            genre_filter: None,
            genre_filter_query: None,
            primary_language_filter: None,
            status_filter: None,
            base_path: "/user/metadata/sports".to_string(),
        };
        match template.render() {
            Ok(reply_html) => (StatusCode::OK, Html(reply_html).into_response()),
            Err(err) => {
                eprintln!("sports metadata template render failed: {err}");
                let template = TemplateError500Context {};
                let reply_html = template.render().unwrap_or_default();
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(reply_html).into_response(),
                )
            }
        }
    }
}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_sports_detail.html")]
struct TemplateMetaSportsDetailContext<'a> {
    template_data: &'a serde_json::Value,
    template_data_exists: &'a bool,
    page_title: Option<String>,
}

pub async fn user_metadata_sports_detail(
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
        let reply_html = template.render().unwrap();
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let template = TemplateMetaSportsDetailContext {
            template_data: &json!({}),
            template_data_exists: &false,
            page_title: Some("MediaKraken Metadata Sports Detail".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}
