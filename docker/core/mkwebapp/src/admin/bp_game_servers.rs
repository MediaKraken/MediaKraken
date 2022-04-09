use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login, AdminUser};
use rocket::serde::{Serialize, Deserialize, json::Json};

#[path = "../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../mk_lib_database_game_servers.rs"]
mod mk_lib_database_game_servers;

#[get("/admin_game_servers/<page>")]
pub async fn admin_game_servers(sqlx_pool: &rocket::State<sqlx::PgPool>, page: i8) -> Template {
    let total_pages: i32 = mk_lib_database_game_servers::mk_lib_database_dedicated_server_count(&sqlx_pool, "".to_string()).await.unwrap() / 30;
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(total_pages, page).await.unwrap();
    let dedicated_server_list =
        mk_lib_database_game_servers::mk_lib_database_dedicated_server_read(&sqlx_pool, 0 ,30).await.unwrap();
    Template::render("bss_admin/bss_admin_game_servers", context! {
        game_info: dedicated_server_list,
        pagination_bar: pagination_html,
    })
}
