#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use stdext::function_name;
use serde_json::json;
use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../mk_lib_database_network_share.rs"]
mod mk_lib_database_network_share;

#[derive(Serialize)]
struct TemplateAdminShareContext {
    template_data: Vec<mk_lib_database_network_share::DBShareList>,
}

#[get("/share")]
pub async fn admin_share(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    user: AdminUser,
) -> Template {
    let share_list =
        mk_lib_database_network_share::mk_lib_database_network_share_read(&sqlx_pool)
            .await
            .unwrap();
    Template::render(
        "bss_admin/bss_admin_share",
        &TemplateAdminShareContext {
            template_data: share_list,
        },
    )
}
