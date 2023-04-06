#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

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
use serde_json::json;
use sqlx::postgres::PgPool;
use stdext::function_name;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../mk_lib_database_network_share.rs"]
mod mk_lib_database_network_share;

#[path = "../mk_lib_database_user.rs"]
mod mk_lib_database_user;

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_share.html")]
struct TemplateAdminShareContext<'a> {
    template_data: &'a Vec<mk_lib_database_network_share::DBShareList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
}

pub async fn admin_share(
    Extension(sqlx_pool): Extension<PgPool>,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(page): Path<i64>,
) -> impl IntoResponse {
    let db_offset: i64 = (page * 30) - 30;
    let total_pages: i64 =
        mk_lib_database_network_share::mk_lib_database_network_share_count(&sqlx_pool)
            .await
            .unwrap();
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
        total_pages,
        page,
        "/admin/admin_share".to_string(),
    )
    .await
    .unwrap();
    let share_list = mk_lib_database_network_share::mk_lib_database_network_share_read(&sqlx_pool)
        .await
        .unwrap();
    let mut template_data_exists = false;
    if share_list.len() > 0 {
        template_data_exists = true;
    }
    let page_usize = page as usize;
    let template = TemplateAdminShareContext {
        template_data: &share_list,
        template_data_exists: &template_data_exists,
        pagination_bar: &pagination_html,
        page: &page_usize,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
