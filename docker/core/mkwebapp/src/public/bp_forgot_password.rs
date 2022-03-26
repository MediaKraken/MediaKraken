use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};

#[get("/forgot_password")]
pub async fn public_forgot_password() -> Template {
    Template::render("bss_public/bss_public_forgot_password", context! {})
}
