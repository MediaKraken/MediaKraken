#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use askama::Template;
use axum::{
    extract::Form,
    extract::Path,
    http::{header, HeaderMap, Method, StatusCode},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Extension, Router,
};
use axum_session::{
    DatabasePool, Session, SessionConfig, SessionLayer, SessionPgPool, SessionStore,
};
use axum_session_auth::*;
use axum_session_auth::{AuthConfig, AuthSession, AuthSessionLayer, Authentication};
use serde_json::json;
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, PgPool,
};
use stdext::function_name;
use validator::Validate;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../mk_lib_database_user.rs"]
mod mk_lib_database_user;

#[derive(Template)]
#[template(path = "bss_public/bss_public_login.html")]
struct LoginTemplate;

pub async fn public_login(auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>) -> impl IntoResponse {
    let template = LoginTemplate {};
    let reply_html = template.render().unwrap();
    auth.login_user(1);
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn perm(
    method: Method,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> String {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database_user::User, i64, PgPool>::build([Method::GET], false)
        .requires(Rights::any([
            Rights::permission("Category::View"),
            Rights::permission("Admin::View"),
        ]))
        .validate(&current_user, &method, None)
        .await
    {
        return format!(
            "User {}, Does not have permissions needed to view this page please login",
            current_user.username
        );
    }

    format!(
        "User has Permissions needed. Here are the Users permissions: {:?}",
        current_user.permissions
    )
}

#[derive(Deserialize)]
pub struct LoginInput {
    email: String,
    password: String,
}

pub async fn public_login_post(Extension(sqlx_pool): Extension<PgPool>, Form(input_data): Form<LoginInput>) -> Redirect {

    Redirect::to("/user/home")
}
