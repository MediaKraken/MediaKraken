use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login};

#[get("/register")]
pub fn public_register() -> Template {
    Template::render("bss_public/bss_public_register", context! {})
}
