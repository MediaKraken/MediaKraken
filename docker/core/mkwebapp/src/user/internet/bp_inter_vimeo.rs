#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use rocket::response::Redirect;
use rocket::Request;
use rocket_auth::{Auth, Error, Login, Signup, User, Users};
use rocket_dyn_templates::{tera::Tera, Template};
use stdext::function_name;
use serde_json::json;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[get("/internet/vimeo")]
pub async fn user_inter_vimeo(user: User) -> Template {
    Template::render(
        "bss_user/internet/bss_user_internet_vimeo",
        tera::Context::new().into_json(),
    )
}

#[get("/internet/vimeo_detail/<guid>")]
pub async fn user_inter_vimeo_detail(user: User, guid: &str) -> Template {
    Template::render(
        "bss_user/internet/bss_user_internet_vimeo_detail",
        tera::Context::new().into_json(),
    )
}
