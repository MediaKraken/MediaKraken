use crate::AppState;
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
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{FromRow, postgres::PgPool};

use crate::mk_lib_database;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_error/bss_error_500.html")]
struct TemplateError500Context {}

#[derive(Debug, Deserialize, Serialize, FromRow)]
struct TemplateMetaPersonList {
    mm_metadata_person_guid: uuid::Uuid,
    mm_metadata_person_name: String,
    mm_metadata_person_image: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
struct MetadataPersonRow {
    mm_metadata_person_guid: uuid::Uuid,
    mm_metadata_person_name: String,
    mm_metadata_person_image: Option<String>,
}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_person.html")]
struct TemplateMetaPersonContext<'a> {
    template_data: &'a Vec<TemplateMetaPersonList>,
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

fn extract_person_image_path(raw: Option<String>) -> String {
    let Some(raw_image) = raw else {
        return String::new();
    };

    let trimmed = raw_image.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    let parsed_json = serde_json::from_str::<serde_json::Value>(trimmed);
    if let Ok(serde_json::Value::Object(image_map)) = parsed_json {
        if let Some(poster) = image_map.get("Poster").and_then(serde_json::Value::as_str) {
            return poster.to_string();
        }
        if let Some(backdrop) = image_map
            .get("Backdrop")
            .and_then(serde_json::Value::as_str)
        {
            return backdrop.to_string();
        }
    }

    raw_image
}

async fn metadata_person_count(pool: &PgPool, starts_with: &str) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar(
        r#"
        select count(*)::bigint
          from mm_metadata_person
         where (
            $1 = ''
            or (
                $1 = '#'
                and upper(left(coalesce(mm_metadata_person_name, ''), 1)) !~ '^[A-Z0-9]'
            )
            or (
                $1 <> '#'
                and upper(left(coalesce(mm_metadata_person_name, ''), 1)) = $1
            )
         )
        "#,
    )
    .bind(starts_with)
    .fetch_one(pool)
    .await
}

async fn metadata_person_read(
    pool: &PgPool,
    starts_with: &str,
    db_offset: i64,
    pagination_count: i64,
) -> Result<Vec<MetadataPersonRow>, sqlx::Error> {
    sqlx::query_as(
        r#"
        select mm_metadata_person_guid,
               coalesce(mm_metadata_person_name, '') as mm_metadata_person_name,
               mm_metadata_person_image
          from mm_metadata_person
         where (
            $1 = ''
            or (
                $1 = '#'
                and upper(left(coalesce(mm_metadata_person_name, ''), 1)) !~ '^[A-Z0-9]'
            )
            or (
                $1 <> '#'
                and upper(left(coalesce(mm_metadata_person_name, ''), 1)) = $1
            )
         )
         order by mm_metadata_person_name asc
         limit $2 offset $3
        "#,
    )
    .bind(starts_with)
    .bind(pagination_count)
    .bind(db_offset)
    .fetch_all(pool)
    .await
}

pub async fn user_metadata_person(
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
        let total_pages: i64 = match metadata_person_count(
            &state.sqlx_pool_ro,
            &starts_with.clone().unwrap_or_default(),
        )
        .await
        {
            Ok(total) => total,
            Err(_) => {
                let template = TemplateError500Context {};
                let reply_html = template.render().unwrap_or_default();
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(reply_html).into_response(),
                );
            }
        };
        let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
            total_pages,
            page,
            "/user/metadata/person".to_string(),
            starts_with.as_deref(),
            pagination_count,
        )
        .await
        .unwrap_or_default();
        let person_list: Vec<TemplateMetaPersonList> = match metadata_person_read(
            &state.sqlx_pool_ro,
            &starts_with.clone().unwrap_or_default(),
            db_offset,
            pagination_count,
        )
        .await
        {
            Ok(rows) => rows,
            Err(_) => {
                let template = TemplateError500Context {};
                let reply_html = template.render().unwrap_or_default();
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(reply_html).into_response(),
                );
            }
        }
        .into_iter()
        .map(|row| TemplateMetaPersonList {
            mm_metadata_person_guid: row.mm_metadata_person_guid,
            mm_metadata_person_name: row.mm_metadata_person_name,
            mm_metadata_person_image: extract_person_image_path(row.mm_metadata_person_image),
        })
        .collect();
        let template_data_exists = !person_list.is_empty();
        let page_usize = page as usize;
        let template = TemplateMetaPersonContext {
            template_data: &person_list,
            template_data_exists: &template_data_exists,
            pagination_bar: &pagination_html,
            page: &page_usize,
            page_title: Some("MediaKraken Metadata Persons".to_string()),
            current: starts_with,
            genre_filter: None,
            genre_filter_query: None,
            primary_language_filter: None,
            status_filter: None,
            base_path: "/user/metadata/person".to_string(),
        };
        let reply_html = template.render().map_err(|e| e.to_string())?;
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_person_detail.html")]
struct TemplateMetaPersonDetailContext {
    template_data: serde_json::Value,
    page_title: Option<String>,
}

pub async fn user_metadata_person_detail(
    State(_state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(_guid): Path<uuid::Uuid>,
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
        let template = TemplateMetaPersonDetailContext {
            template_data: json!({}),
            page_title: Some("MediaKraken Metadata Person Detail".to_string()),
        };
        let reply_html = template.render().map_err(|e| e.to_string())?;
        (StatusCode::OK, Html(reply_html).into_response())
    }
}
