#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use stdext::function_name;
use serde_json::json;
use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[get("/login")]
pub async fn public_login() -> Template {
    Template::render(
        "bss_public/bss_public_login.html",
        tera::Context::new().into_json(),
    )
}

#[post("/login", data = "<form>")]
pub async fn public_login_post(auth: Auth<'_>, form: Form<Login>) -> Result<Redirect, Error> {
    let result = auth.login(&form).await;
    result?;
    Ok(Redirect::to("/user/home"))
}
