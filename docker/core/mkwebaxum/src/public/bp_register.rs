use askama::Template;
use axum::{
    extract::Form,
    http::{Method, StatusCode},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Extension,
};
use axum_flash::{Flash, IncomingFlashes, Key};
use axum_session::{SessionConfig, SessionLayer};
use axum_session_sqlx::{SessionPgPool};
use axum_session_auth::*;
use crate::mk_lib_database;
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, PgPool,
};
use validator::Validate;

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
    username: String,
    password: String,
}

pub async fn public_register_post(
    Extension(ReadWritePool(sqlx_pool_rw)): Extension<ReadWritePool>,
    Extension(ReadOnlyPool(sqlx_pool_ro)): Extension<ReadOnlyPool>,
    mut flash: Flash,
    Form(input_data): Form<RegisterInput>,
) -> Redirect {
    let user_found = mk_lib_database::mk_lib_database_user::mk_lib_database_user_exists(
        &sqlx_pool_ro,
        &input_data.username,
    )
    .await
    .unwrap();
    if user_found == true {
        flash.error("User already exists!");
    } else {
        let user_id: i64 = mk_lib_database::mk_lib_database_user::mk_lib_database_user_insert(
            &sqlx_pool_rw,
            &input_data.username,
            &input_data.password,
        )
        .await
        .unwrap();
    // using rw here as I need to see the insert immediately
        if mk_lib_database::mk_lib_database_user::mk_lib_database_user_count(
            &sqlx_pool_rw,
            String::new(),
        )
        .await
        .unwrap()
            == 2
        // Use 2 as 1 is guest
        {
            let _result = mk_lib_database::mk_lib_database_user::mk_lib_database_user_set_admin(
                &sqlx_pool_rw, user_id,
            )
            .await;
        }
    }
    Redirect::to("/public/login")
}
