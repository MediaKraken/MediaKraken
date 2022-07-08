use rocket::response::Redirect;
use rocket::Request;
use rocket_dyn_templates::{tera::Tera, Template};

#[get("/about")]
pub async fn public_about() -> Template {
    Template::render(
        "bss_public/bss_public_about",
        tera::Context::new().into_json(),
    )
}
