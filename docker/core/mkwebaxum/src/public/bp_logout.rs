use crate::AppState;
use crate::mk_lib_database;
use axum::extract::State;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use sqlx::postgres::PgPool;

pub async fn public_logout(
    State(state): State<AppState>,
    mut auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    let _result = mk_lib_database::mk_lib_database_user::mk_lib_database_user_logout(
        &state.sqlx_pool_rw,
        current_user.id,
    )
    .await;
    // Destroy the server-side session and clear its cookie on response so a
    // stolen cookie cannot be reused.
    auth.session.destroy();
    auth.logout_user();
    Redirect::to("/public/login")
}
