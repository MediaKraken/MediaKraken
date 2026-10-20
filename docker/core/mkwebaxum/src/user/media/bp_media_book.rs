use crate::AppState;
use crate::mk_lib_database;
use crate::user_preferences;
use askama::Template;
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
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(sqlx::FromRow)]
struct MediaBookListRow {
    mm_metadata_book_guid: uuid::Uuid,
    mm_metadata_book_name: String,
    mm_metadata_book_cover: String,
}

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_book.html")]
struct TemplateMediaBookContext<'a> {
    template_data: &'a Vec<MediaBookListRow>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
    page_title: Option<String>,
}

pub async fn user_media_book(
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
        let total_pages: i64 =
        mk_lib_database::database_media::mk_lib_database_media_book::mk_lib_database_media_book_count(
           &state.sqlx_pool_ro,
            String::new(),
        )
        .await
        .unwrap();
        let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
            total_pages,
            page,
            "/user/media/book".to_string(),
            None,
            pagination_count,
        )
        .await
        .unwrap();
        let book_list = sqlx::query_as::<_, MediaBookListRow>(
            r#"select b.mm_metadata_book_guid,
                      b.mm_metadata_book_name,
                      CASE WHEN COALESCE(b.mm_metadata_book_localimage->>'Poster',
                                        b.mm_metadata_book_localimage->'Images'->>'Poster', '') = ''
                           THEN ''
                           ELSE '/metadata' || COALESCE(b.mm_metadata_book_localimage->>'Poster',
                                                       b.mm_metadata_book_localimage->'Images'->>'Poster')
                      END as mm_metadata_book_cover
               from mm_metadata_book b, mm_media m
               where m.mm_media_metadata_guid = b.mm_metadata_book_guid
               order by LOWER(b.mm_metadata_book_name)
               offset $1 limit $2"#,
        )
        .bind(db_offset)
        .bind(pagination_count)
        .fetch_all(&state.sqlx_pool_ro)
        .await
        .unwrap();
        let mut template_data_exists = false;
        if book_list.len() > 0 {
            template_data_exists = true;
        }
        let page_usize = page as usize;
        let template = TemplateMediaBookContext {
            template_data: &book_list,
            template_data_exists: &template_data_exists,
            pagination_bar: &pagination_html,
            page: &page_usize,
            page_title: Some("MediaKraken Books".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

#[derive(sqlx::FromRow)]
struct MediaBookDetailRow {
    mm_metadata_book_json: serde_json::Value,
    cover_path: String,
}

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_book_detail.html")]
struct TemplateMediaBookDetailContext {
    template_data: serde_json::Value,
    page_title: Option<String>,
}

pub async fn user_media_book_detail(
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
        let row = sqlx::query_as::<_, MediaBookDetailRow>(
            r#"select mm_metadata_book_json,
                      COALESCE(mm_metadata_book_localimage->>'Poster',
                               mm_metadata_book_localimage->'Images'->>'Poster', '') as cover_path
               from mm_metadata_book
               where mm_metadata_book_guid = $1"#,
        )
        .bind(guid)
        .fetch_optional(&state.sqlx_pool_ro)
        .await;
        let template_data = match row {
            Ok(Some(row)) => {
                crate::user::detail_map::book(&row.mm_metadata_book_json, &row.cover_path)
            }
            Ok(None) => {
                return (
                    StatusCode::NOT_FOUND,
                    Html(String::from("Not found")).into_response(),
                );
            }
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(String::from("Internal server error")).into_response(),
                );
            }
        };
        let template = TemplateMediaBookDetailContext {
            template_data,
            page_title: Some("MediaKraken Book Detail".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}
