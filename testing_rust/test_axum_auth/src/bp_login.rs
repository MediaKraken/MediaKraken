#![cfg_attr(debug_assertions, allow(dead_code))]

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
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, PgPool,
};
use stdext::function_name;
use validator::Validate;

#[path = "mk_lib_database_user.rs"]
mod mk_lib_database_user;

// pub async fn greet(
//     auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
// ) -> String {
//     format!(
//         "Hello {}, Try logging in via /login or testing permissions via /perm",
//         auth.current_user.unwrap().username
//     )
// }

pub async fn login(
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> String {
    auth.login_user(2);
    "You are logged in as a User please try /perm to check permissions".to_owned()
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

#[derive(Template)]
#[template(path = "bss_public_login.html")]
struct LoginTemplate;

pub async fn public_login() -> impl IntoResponse {
    let template = LoginTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Deserialize)]
pub struct LoginInput {
    username: String,
    password: String,
}

pub async fn public_login_post(
    Extension(sqlx_pool): Extension<PgPool>,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Form(input_data): Form<LoginInput>,
) -> Redirect {
    let user_id: i64 = mk_lib_database_user::mk_lib_database_user_login_verification(
        &sqlx_pool,
        &input_data.username,
        &input_data.password,
    )
    .await
    .unwrap();
    // TODO show error when not found
    if user_id > 0 {
        mk_lib_database_user::mk_lib_database_user_login(&sqlx_pool, user_id).await;
        auth.login_user(user_id);
    }
    Redirect::to("/user/home")
}
