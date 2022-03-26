use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};

#[get("/login")]
pub async fn public_login() -> Template {
    Template::render("bss_public/bss_public_login", context! {})
}
