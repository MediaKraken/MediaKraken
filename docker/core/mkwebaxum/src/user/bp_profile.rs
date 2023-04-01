#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use serde_json::json;
use stdext::function_name;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../mk_lib_database_user_profile.rs"]
mod mk_lib_database_user_profile;

#[derive(Template)]
#[template(path = "bss_user/bss_user_profile.html")]
struct UserProfileTemplate;

pub async fn user_profile() -> impl IntoResponse {
    let template = UserProfileTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

/*
@blueprint_user_profile.route('/user_profile', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/bss_user_profile.html')
@common_global.auth.login_required
pub async fn url_bp_user_profile(request):
    return {}

 */
