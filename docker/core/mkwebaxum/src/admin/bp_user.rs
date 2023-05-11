use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use axum_session_auth::*;
use axum_session_auth::{AuthConfig, AuthSession, AuthSessionLayer, Authentication};
use mk_lib_common::mk_lib_common_pagination;
use mk_lib_database;
use mk_lib_logging::mk_lib_logging;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgPool;
use stdext::function_name;

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_user.html")]
struct TemplateAdminUserContext<'a> {
    template_data: &'a Vec<mk_lib_database::mk_lib_database_user::DBUserList>,
    pagination_bar: &'a String,
    page: &'a usize,
}

pub async fn admin_user(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(page): Path<i64>,
) -> impl IntoResponse {
    let db_offset: i64 = (page * 30) - 30;
    let total_pages: i64 = mk_lib_database::mk_lib_database_user::mk_lib_database_user_count(
        &sqlx_pool,
        String::new(),
    )
    .await
    .unwrap();
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
        total_pages,
        page,
        "/admin/user".to_string(),
    )
    .await
    .unwrap();
    let user_list =
        mk_lib_database::mk_lib_database_user::mk_lib_database_user_read(&sqlx_pool, db_offset, 30)
            .await
            .unwrap();
    let page_usize = page as usize;
    let template = TemplateAdminUserContext {
        template_data: &user_list,
        pagination_bar: &pagination_html,
        page: &page_usize,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_user_detail.html")]
struct TemplateAdminUserDetailContext {}

pub async fn admin_user_detail(
    Extension(sqlx_pool): Extension<PgPool>,
    Path(guid): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let template = TemplateAdminUserDetailContext {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_user_delete.html")]
struct TemplateAdminUserDeleteContext {}

pub async fn admin_user_delete(
    Extension(sqlx_pool): Extension<PgPool>,
    Path(guid): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let template = TemplateAdminUserDeleteContext {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
