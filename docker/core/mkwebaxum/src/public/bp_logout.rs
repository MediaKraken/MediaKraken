use askama::Template;
use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Extension,
};
use axum_session_auth::{Auth, AuthSession, Rights, SessionPgPool};
use crate::mk_lib_database;
use serde_json::json;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, PgPool,
};

pub async fn public_logout(
    Extension(sqlx_pool): Extension<PgPool>,
    mut auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    let _result = mk_lib_database::mk_lib_database_user::mk_lib_database_user_logout(
        &sqlx_pool,
        current_user.id,
    )
    .await;
    auth.logout_user();
    Redirect::to("/public/login")
}
