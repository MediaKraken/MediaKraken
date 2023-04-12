use axum::http::Method;
use axum_session::SessionPgPool;
use axum_session_auth::*;
use sqlx::PgPool;

use crate::mk_lib_database_user;

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
