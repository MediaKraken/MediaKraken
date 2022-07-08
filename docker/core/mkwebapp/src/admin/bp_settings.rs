use rocket::response::Redirect;
use rocket::Request;
use rocket_auth::{AdminUser, Auth, Error, Login, Signup, Users};
use rocket_dyn_templates::{tera::Tera, Template};

#[get("/admin_settings")]
pub async fn admin_settings(sqlx_pool: &rocket::State<sqlx::PgPool>, user: AdminUser) -> Template {
    Template::render(
        "bss_admin/bss_admin_settings",
        tera::Context::new().into_json(),
    )
}
