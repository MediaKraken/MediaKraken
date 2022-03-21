use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};

#[get("/login")]
pub fn public_login() -> Template {
    Template::render("bss_public/bss_public_login", context! {})
}
