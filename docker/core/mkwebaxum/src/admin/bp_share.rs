#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use stdext::function_name;
use serde_json::json;
use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use sqlx::postgres::PgPool;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../mk_lib_database_network_share.rs"]
mod mk_lib_database_network_share;

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_share.html")]
struct TemplateAdminShareContext {
    template_data: Vec<mk_lib_database_network_share::DBShareList>,
}

pub async fn admin_share(Extension(sqlx_pool): Extension<PgPool>) -> impl IntoResponse {
    let share_list =
        mk_lib_database_network_share::mk_lib_database_network_share_read(&sqlx_pool)
            .await
            .unwrap();
    let template = TemplateAdminShareContext {
        template_data_db_version: &share_list,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
