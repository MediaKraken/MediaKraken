use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::Request;
use rocket_auth::{AdminUser, Auth, Error, Login, Signup, Users};
use rocket_dyn_templates::{tera::Tera, Template};
use serde_json::json;
use stdext::function_name;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../mk_lib_database_game_servers.rs"]
mod database::mk_lib_database_game_servers;

#[derive(Serialize)]
struct TemplateAdminGameServersContext {
    template_data: Vec<mk_lib_database_game_servers::DBGameServerList>,
    pagination_bar: String,
}

#[get("/game_servers/<page>")]
pub async fn admin_game_servers(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    user: AdminUser,
    page: i32,
) -> Template {
    let db_offset: i64 = (page * 30) - 30;
    let total_pages: i64 =
        database::mk_lib_database_game_servers::mk_lib_database_game_server_count(&sqlx_pool, String::new())
            .await
            .unwrap();
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
        total_pages,
        page,
        "/admin/game_servers".to_string(),
    )
    .await
    .unwrap();
    let dedicated_server_list = database::mk_lib_database_game_servers::mk_lib_database_game_server_read(
        &sqlx_pool,
        String::new(),
        db_offset,
        30,
    )
    .await
    .unwrap();
    Template::render(
        "bss_admin/bss_admin_game_servers",
        &TemplateAdminGameServersContext {
            template_data: dedicated_server_list,
            pagination_bar: pagination_html,
        },
    )
}
