#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use stdext::function_name;
use serde_json::json;
use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};

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
