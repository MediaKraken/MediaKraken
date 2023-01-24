#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use rocket::response::Redirect;
use rocket::Request;
use rocket_dyn_templates::{tera::Tera, Template};
use stdext::function_name;
use serde_json::json;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[get("/about")]
pub async fn public_about() -> Template {
    Template::render(
        "bss_public/bss_public_about",
        tera::Context::new().into_json(),
    )
}
