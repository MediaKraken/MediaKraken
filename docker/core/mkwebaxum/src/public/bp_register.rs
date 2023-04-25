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
use axum_session_auth::{AuthConfig, AuthSession, AuthSessionLayer, Authentication};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, PgPool,
};
use stdext::function_name;
use validator::Validate;

use crate::mk_lib_logging;

use crate::database::mk_lib_database_user;

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
    let user_found =
        mk_lib_database_user::mk_lib_database_user_exists(&sqlx_pool, &input_data.email)
            .await
            .unwrap();
    if user_found == true {
        // TODO flash error
    } else {
        let user_id: i64 = mk_lib_database_user::mk_lib_database_user_insert(
            &sqlx_pool,
            &input_data.email,
            &input_data.password,
        )
        .await
        .unwrap();
        if mk_lib_database_user::mk_lib_database_user_count(&sqlx_pool, String::new())
            .await
            .unwrap()
            == 2
        // 2 as 1 is guest
        {
            mk_lib_database_user::mk_lib_database_user_set_admin(&sqlx_pool, user_id).await;
        }
    }
    Redirect::to("/public/login")
}
