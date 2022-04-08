use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login, AdminUser};
use paginator::{Paginator, PageItem};
use core::fmt::Write;

#[path = "../mk_lib_database_game_servers.rs"]
mod mk_lib_database_game_servers;

#[get("/admin_game_servers")]
pub async fn admin_game_servers(sqlx_pool: &rocket::State<sqlx::PgPool>) -> Template {
    let dedicated_server_list =
        mk_lib_database_game_servers::mk_lib_database_dedicated_server_read(&sqlx_pool, 0 ,30).await.unwrap();
    Template::render("bss_admin/bss_admin_game_servers", context! {})
}
