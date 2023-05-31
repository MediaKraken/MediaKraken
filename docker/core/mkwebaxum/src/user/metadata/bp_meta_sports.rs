use askama::Template;
use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    Extension,
};
use axum_session_auth::{AuthSession, SessionPgPool};
use mk_lib_common::mk_lib_common_pagination;
use mk_lib_database;
use serde_json::json;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_sports.html")]
struct TemplateMetaSportsContext<'a> {
    template_data: &'a Vec<
        mk_lib_database::database_metadata::mk_lib_database_metadata_sports::DBMetaSportsList,
    >,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
}

pub async fn user_metadata_sports(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(page): Path<i64>,
) -> impl IntoResponse {
    let db_offset: i64 = (page * 30) - 30;
    let total_pages: i64 =
        mk_lib_database::database_metadata::mk_lib_database_metadata_sports::mk_lib_database_metadata_sports_count(
            &sqlx_pool,
            String::new(),
        )
        .await
        .unwrap();
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
        total_pages,
        page,
        "/user/metadata/sports".to_string(),
    )
    .await
    .unwrap();
    let sports_list =
        mk_lib_database::database_metadata::mk_lib_database_metadata_sports::mk_lib_database_metadata_sports_read(
            &sqlx_pool,
            String::new(),
            db_offset,
            30,
        )
        .await
        .unwrap();
    let mut template_data_exists = false;
    if sports_list.len() > 0 {
        template_data_exists = true;
    }
    let page_usize = page as usize;
    let template = TemplateMetaSportsContext {
        template_data: &sports_list,
        template_data_exists: &template_data_exists,
        pagination_bar: &pagination_html,
        page: &page_usize,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_sports_detail.html")]
struct TemplateMetaSportsDetailContext<'a> {
    template_data: &'a serde_json::Value,
    template_data_exists: &'a bool,
}

pub async fn user_metadata_sports_detail(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(guid): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let template = TemplateMetaSportsDetailContext {
        template_data: &json!({}),
        template_data_exists: &false,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
