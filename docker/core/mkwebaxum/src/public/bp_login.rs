use askama::Template;
use axum::{
    extract::Form,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use axum_flash::{Flash, IncomingFlashes, Key};
use axum_session::SessionPgPool;
use axum_session_auth::*;
use crate::mk_lib_database;
use serde::Deserialize;
use sqlx::PgPool;
use validator::Validate;

#[derive(Template)]
#[template(path = "bss_public/bss_public_login.html")]
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
    mut auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    mut flash: Flash,
    Form(input_data): Form<LoginInput>,
) -> Redirect {
    let user_id: i64 =
        mk_lib_database::mk_lib_database_user::mk_lib_database_user_login_verification(
            &sqlx_pool,
            &input_data.username,
            &input_data.password,
        )
        .await
        .unwrap();
    if user_id > 0 {
        println!("Login User {:?}", user_id);
        let _result =
            mk_lib_database::mk_lib_database_user::mk_lib_database_user_login(&sqlx_pool, user_id)
                .await;
        auth.login_user(user_id);
        auth.remember_user(true);
    } else {
        flash.error("Unknown user or password incorrect!");
    }
    Redirect::to("/user/home")
}
