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

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[derive(Template)]
#[template(path = "bss_user/internet/bss_user_internet_flickr.html")]
struct TemplateUserInternetFlickr<'a> {
    template_data: &'a Vec<mk_lib_database_cron::DBCronList>,
    template_data_exists: &'a bool,
}

pub async fn user_inter_flickr() -> impl IntoResponse {
    let template = TemplateUserInternetFlickr {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_user/internet/bss_user_internet_flickr_detail.html")]
struct TemplateUserInternetFlickrDetail;

pub async fn user_inter_flickr_detail() -> impl IntoResponse {
    let template = TemplateUserInternetFlickrDetail {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
