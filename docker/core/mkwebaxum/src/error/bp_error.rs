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

#[catch(401)]
pub async fn general_not_authorized() -> Template {
    Template::render("bss_error/bss_error_401.html", tera::Context::new().into_json())
}

#[catch(403)]
pub async fn general_not_administrator() -> Template {
    Template::render("bss_error/bss_error_403.html", tera::Context::new().into_json())
}

#[catch(404)]
pub async fn general_not_found() -> Template {
    Template::render("bss_error/bss_error_404.html", tera::Context::new().into_json())
}

#[catch(500)]
pub async fn general_security() -> Template {
    Template::render("bss_error/bss_error_500.html", tera::Context::new().into_json())
}
