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

#[get("/logout")]
pub async fn public_logout(auth: Auth<'_>) -> Result<Template, Error> {
    auth.logout()?;
    Ok(Template::render("logout", tera::Context::new().into_json()))
}
