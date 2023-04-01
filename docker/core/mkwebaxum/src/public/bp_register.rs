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
use axum_session_auth::{AuthConfig, AuthSession, AuthSessionLayer, Authentication};
use serde::{Deserialize, Serialize};
use serde_json::json;
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
#[template(path = "bss_public/bss_public_register.html")]
struct RegisterTemplate;

pub async fn public_register() -> impl IntoResponse {
    let template = RegisterTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Deserialize)]
pub struct RegisterInput {
    email: String,
    password: String,
}

pub async fn public_register_post(
    Extension(sqlx_pool): Extension<PgPool>,
    Form(input_data): Form<RegisterInput>,
) -> Redirect {
    let user_found = mk_lib_database_user::mk_lib_database_user_exists(&sqlx_pool, &input_data.email).await.unwrap();
    if user_found == true {
        // TODO flash error
    } else {
        mk_lib_database_user::mk_lib_database_user_insert(&sqlx_pool, &input_data.email, &input_data.password).await;
        // TODO insert base rights
        if mk_lib_database_user::mk_lib_database_user_count(&sqlx_pool, String::new())
        .await
        .unwrap()
        == 1
    {
        mk_lib_database_user::mk_lib_database_user_set_admin(&sqlx_pool).await;
    }
    }
    Redirect::to("/public/login")
}
