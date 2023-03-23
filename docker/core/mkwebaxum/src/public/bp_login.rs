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
use validator::Validate;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[derive(Template)]
#[template(path = "bss_public/bss_public_login.html")]
struct LoginTemplate;

pub async fn public_login() -> impl IntoResponse {
    let template = LoginTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

// #[post("/login", data = "<form>")]
// pub async fn public_login_post(auth: Auth<'_>, form: Form<Login>) -> Result<Redirect, Error> {
//     let result = auth.login(&form).await;
//     result?;
//     Ok(Redirect::to("/user/home"))
// }
