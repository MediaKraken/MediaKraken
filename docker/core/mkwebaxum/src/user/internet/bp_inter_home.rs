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

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

use crate::mk_lib_database_user;

#[derive(Template)]
#[template(path = "bss_user/internet/bss_user_internet.html")]
struct UserInternetTemplate;

pub async fn user_inter_home(
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let template = UserInternetTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
