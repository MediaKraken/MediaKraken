#![cfg_attr(debug_assertions, allow(dead_code))]

use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use axum_session_auth::*;
use axum_session_auth::{AuthConfig, AuthSession, AuthSessionLayer, Authentication};
use serde_json::json;
use sqlx::postgres::PgPool;
use stdext::function_name;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../mk_lib_database_user_profile.rs"]
mod mk_lib_database_user_profile;

#[path = "../mk_lib_database_user.rs"]
mod mk_lib_database_user;

#[derive(Template)]
#[template(path = "bss_user/bss_user_profile.html")]
struct UserProfileTemplate;

pub async fn user_profile(
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
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
