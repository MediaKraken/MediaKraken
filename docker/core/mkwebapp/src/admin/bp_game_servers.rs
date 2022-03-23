use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login, AdminUser};
use paginate::Pages;

#[get("/admin_game_servers")]
pub fn admin_game_servers() -> Template {
    Template::render("bss_admin/bss_admin_game_servers", context! {})
}
