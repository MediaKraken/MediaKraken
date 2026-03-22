use crate::AppState;
use crate::mk_lib_database;
use askama::Template;
use axum::{
    extract::{Form, State},
    response::{Html, IntoResponse, Redirect},
};
use axum_flash::{Flash, IncomingFlashes};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use serde::Deserialize;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_public/bss_public_login.html")]
struct LoginTemplate {
    flash_messages: Vec<String>,
}

pub async fn public_login(flashes: IncomingFlashes) -> impl IntoResponse {
    let template = LoginTemplate {
        flash_messages: flashes
            .iter()
            .map(|(_, message)| message.to_string())
            .collect(),
    };
    let reply_html = template.render().unwrap();
    (flashes, Html(reply_html))
}

#[derive(Deserialize)]
pub struct LoginInput {
    username: String,
    password: String,
}

pub async fn public_login_post(
    State(state): State<AppState>,
    mut auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    flash: Flash,
    Form(input_data): Form<LoginInput>,
) -> (Flash, Redirect) {
    let user_id: i64 =
        mk_lib_database::mk_lib_database_user::mk_lib_database_user_login_verification(
            &state.sqlx_pool_rw,
            &input_data.username,
            &input_data.password,
        )
        .await
        .unwrap();
    if user_id > 0 {
        println!("Login User {:?}", user_id);
        let _result = mk_lib_database::mk_lib_database_user::mk_lib_database_user_login(
            &state.sqlx_pool_rw,
            user_id,
        )
        .await;
        auth.login_user(user_id);
        auth.remember_user(true);
        (flash, Redirect::to("/user/home"))
    } else {
        (
            flash.error("Unknown user or password incorrect!"),
            Redirect::to("/public/login"),
        )
    }
}
