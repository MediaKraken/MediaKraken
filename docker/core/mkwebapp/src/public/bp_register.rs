use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};

#[get("/register")]
pub async fn public_register() -> Template {
    Template::render("bss_public/bss_public_register", {})
}
