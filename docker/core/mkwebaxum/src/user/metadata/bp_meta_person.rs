use crate::AppState;
use crate::user_preferences;
use askama::Template;
use axum::{
    extract::{Path, State},
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

#[derive(Debug, Deserialize, Serialize, FromRow)]
struct TemplateMetaPersonList {
    mm_metadata_person_guid: uuid::Uuid,
    mm_metadata_person_name: String,
    mm_metadata_person_image: String,
}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_person.html")]
struct TemplateMetaPersonContext<'a> {
    template_data: &'a Vec<TemplateMetaPersonList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
    page_title: Option<String>,
}

pub async fn user_metadata_person(
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
        let template = TemplateError401Context {};
        let reply_html = template.render().unwrap();
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let pagination_count =
            user_preferences::load_user_pagination_count(&state.sqlx_pool_ro, current_user.id)
                .await
                .unwrap_or(user_preferences::DEFAULT_PAGINATION_COUNT);
        let db_offset: i64 = (page * pagination_count) - pagination_count;
        let total_pages: i64 = mk_lib_database::database_metadata::mk_lib_database_metadata_person::mk_lib_database_metadata_person_count(
            &state.sqlx_pool_ro,
            String::new(),
        )
        .await
        .unwrap();
        let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
            total_pages,
            page,
            "/user/metadata/person".to_string(),
            None,
            pagination_count,
        )
        .await
        .unwrap();
        let person_list: Vec<TemplateMetaPersonList> = sqlx::query_as(
            r#"select mm_metadata_person_guid,
            mm_metadata_person_name,
            COALESCE(mm_metadata_person_image->>'Poster', '') as mm_metadata_person_image
            from mm_metadata_person
            order by LOWER(mm_metadata_person_name)
            offset $1 limit $2"#,
        )
        .bind(db_offset)
        .bind(30_i64)
        .fetch_all(&state.sqlx_pool_ro)
        .await
        .unwrap();
        let template_data_exists = !person_list.is_empty();
        let page_usize = page as usize;
        let template = TemplateMetaPersonContext {
            template_data: &person_list,
            template_data_exists: &template_data_exists,
            pagination_bar: &pagination_html,
            page: &page_usize,
            page_title: Some("MediaKraken Metadata Persons".to_string()),
        };
        let reply_html = template.render().unwrap();
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
        let reply_html = template.render().unwrap();
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let template = TemplateMetaPersonDetailContext {
            template_data: json!({}),
            page_title: Some("MediaKraken Metadata Person Detail".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}
