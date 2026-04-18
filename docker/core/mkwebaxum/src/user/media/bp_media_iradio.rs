use crate::AppState;
use crate::mk_lib_database;
use crate::user_preferences;
use askama::Template;
use axum::extract::{Path, State};
use axum::{
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use mk_lib_common::mk_lib_common_pagination;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_iradio.html")]
struct TemplateMediaIradioContext<'a> {
    template_data:
        &'a Vec<mk_lib_database::database_media::mk_lib_database_media_iradio::DBMediaIradioList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page_title: Option<String>,
}

pub async fn user_media_iradio(
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
        match template.render() {
            Ok(reply_html) => (StatusCode::UNAUTHORIZED, Html(reply_html).into_response()),
            Err(_) => (
                StatusCode::UNAUTHORIZED,
                Html(String::new()).into_response(),
            ),
        }
    } else {
        let pagination_count =
            user_preferences::load_user_pagination_count(&state.sqlx_pool_ro, current_user.id)
                .await
                .unwrap_or(user_preferences::DEFAULT_PAGINATION_COUNT);
        let db_offset: i64 = (page * pagination_count) - pagination_count;

        let total_rows = mk_lib_database::database_media::mk_lib_database_media_iradio::mk_lib_database_media_iradio_count(
            &state.sqlx_pool_ro,
            true,
            None,
        )
        .await
        .unwrap_or(0);
        let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
            total_rows,
            page,
            "/user/media/iradio".to_string(),
            None,
            pagination_count,
        )
        .await
        .unwrap_or_default();

        let stations = mk_lib_database::database_media::mk_lib_database_media_iradio::mk_lib_database_media_iradio_read(
            &state.sqlx_pool_ro,
            db_offset,
            Some(pagination_count),
            true,
            None,
        )
        .await
        .unwrap_or_default();

        let template_data_exists = !stations.is_empty();
        let template = TemplateMediaIradioContext {
            template_data: &stations,
            template_data_exists: &template_data_exists,
            pagination_bar: &pagination_html,
            page_title: Some("MediaKraken iRadio".to_string()),
        };

        match template.render() {
            Ok(reply_html) => (StatusCode::OK, Html(reply_html).into_response()),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html(String::from("Failed to render iRadio page")).into_response(),
            ),
        }
    }
}
