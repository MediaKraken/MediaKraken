use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};

#[get("/about")]
pub fn public_about() -> Template {
    Template::render("bss_public/bss_public_about", context! {})
}
