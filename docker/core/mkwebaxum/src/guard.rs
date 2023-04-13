#![cfg_attr(debug_assertions, allow(dead_code))]

use axum::http::Method;
use axum::response::{IntoResponse, Redirect};
use axum_session::SessionPgPool;
use axum_session_auth::*;
use sqlx::PgPool;
use stdext::function_name;

use crate::mk_lib_database_user;

pub async fn guard_page_by_user(
    user_admin: bool,
    method: Method,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> Result<bool, Box<dyn std::error::Error>> {
    let current_user = auth.current_user.clone().unwrap_or_default();
    let mut has_rights = true;
    if user_admin == true {
        if !Auth::<mk_lib_database_user::User, i64, PgPool>::build([Method::GET], false)
            .requires(Rights::any([
                Rights::permission("Admin::View"),
            ]))
            .validate(&current_user, &method, None)
            .await
        {
//            Redirect::to("/error/403");
            has_rights = false;
        }
    } else {
        if !Auth::<mk_lib_database_user::User, i64, PgPool>::build([Method::GET], false)
            .requires(Rights::any([
                Rights::permission("Category::View"),
                Rights::permission("User::View"),
            ]))
            .validate(&current_user, &method, None)
            .await
        {
            //Redirect::to("/error/401");
            has_rights = false;
        }
    }
    Ok(has_rights)
}
