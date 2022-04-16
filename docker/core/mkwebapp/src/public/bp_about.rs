use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera};

#[get("/about")]
pub async fn public_about() -> Template {
    Template::render("bss_public/bss_public_about", tera::Context::new().into_json())
}
