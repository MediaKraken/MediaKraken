#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use rocket::response::Redirect;
use rocket::Request;
use rocket_auth::{Auth, Error, Login, Signup, User, Users};
use rocket_dyn_templates::{tera::Tera, Template};

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[get("/internet/flickr")]
pub async fn user_inter_flickr(user: User) -> Template {
    Template::render(
        "bss_user/internet/bss_user_internet_flickr",
        tera::Context::new().into_json(),
    )
}

#[get("/internet/flickr_detail/<guid>")]
pub async fn user_inter_flickr_detail(user: User, guid: &str) -> Template {
    Template::render(
        "bss_user/internet/bss_user_internet_flickr_detail",
        tera::Context::new().into_json(),
    )
}
