#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use stdext::function_name;
use serde_json::json;
use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../mk_lib_database_user.rs"]
mod mk_lib_database_user;

#[get("/register")]
pub async fn public_register() -> Template {
    Template::render(
        "bss_public/bss_public_register",
        tera::Context::new().into_json(),
    )
}

#[post("/register", data = "<form>")]
pub async fn public_register_post(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    auth: Auth<'_>,
    form: Form<Signup>,
) -> Result<Redirect, Error> {
    auth.signup(&form).await?;
    auth.login(&form.into()).await?;
    if mk_lib_database_user::mk_lib_database_user_count(&sqlx_pool, String::new())
        .await
        .unwrap()
        == 1
    {
        mk_lib_database_user::mk_lib_database_user_set_admin(&sqlx_pool).await;
    }
    Ok(Redirect::to("/user/home"))
}