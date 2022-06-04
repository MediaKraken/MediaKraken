use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera};
use rocket_auth::{Users, Error, Auth, Signup, Login, AdminUser};

#[get("/admin_settings")]
pub async fn admin_settings(sqlx_pool: &rocket::State<sqlx::PgPool>, user: AdminUser) -> Template {
    Template::render("bss_admin/bss_admin_settings", tera::Context::new().into_json())
}