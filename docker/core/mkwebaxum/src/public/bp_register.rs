use crate::AppState;
use crate::mk_lib_database;
use askama::Template;
use axum::{
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use axum_flash::{Flash, IncomingFlashes};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "bss_public/bss_public_register.html")]
struct RegisterTemplate {
    flash_messages: Vec<String>,
}

pub async fn public_register(flashes: IncomingFlashes) -> impl IntoResponse {
    let template = RegisterTemplate {
        flash_messages: flashes
            .iter()
            .map(|(_, message)| message.to_string())
            .collect(),
    };
    let reply_html = template.render().unwrap();
    (flashes, StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Deserialize)]
pub struct RegisterInput {
    username: String,
    password: String,
}

pub async fn public_register_post(
    State(state): State<AppState>,
    flash: Flash,
    Form(input_data): Form<RegisterInput>,
) -> (Flash, Redirect) {
    let user_found = mk_lib_database::mk_lib_database_user::mk_lib_database_user_exists(
        &state.sqlx_pool_ro,
        &input_data.username,
    )
    .await
    .unwrap();
    if user_found {
        (
            flash.error("User already exists!"),
            Redirect::to("/public/register"),
        )
    } else {
        let user_id: i64 = mk_lib_database::mk_lib_database_user::mk_lib_database_user_insert(
            &state.sqlx_pool_rw,
            &input_data.username,
            &input_data.password,
        )
        .await
        .unwrap();
        // using rw here as I need to see the insert immediately
        if mk_lib_database::mk_lib_database_user::mk_lib_database_user_count(
            &state.sqlx_pool_rw,
            String::new(),
        )
        .await
        .unwrap()
            == 2
        // Use 2 as 1 is guest
        {
            let _result = mk_lib_database::mk_lib_database_user::mk_lib_database_user_set_admin(
                &state.sqlx_pool_rw,
                user_id,
            )
            .await;
        }

        (flash, Redirect::to("/public/login"))
    }
}
