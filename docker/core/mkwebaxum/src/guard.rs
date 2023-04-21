#![cfg_attr(debug_assertions, allow(dead_code))]

use axum::http::Method;
use axum_session::SessionPgPool;
use axum_session_auth::*;
use sqlx::PgPool;
use stdext::function_name;
use serde_json::json;

use crate::bp_error;

use crate::error_handling;

use crate::mk_lib_logging;

use crate::mk_lib_database_user;

pub async fn guard_page_by_user(
    user_admin: bool,
    method: Method,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> Result<(), error_handling::MKAxumError> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let current_user = auth.current_user.clone().unwrap_or_default();
    if user_admin == true {
        if !Auth::<mk_lib_database_user::User, i64, PgPool>::build([Method::GET], false)
            .requires(Rights::any([
                Rights::permission("Admin::View"),
            ]))
            .validate(&current_user, &method, None)
            .await
        {
            println!("here I am");
            return Err(error_handling::MKAxumError::Error403);
            //bp_error::general_not_administrator().await;
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
            println!("here I am 2");
            return Err(error_handling::MKAxumError::Error401);
            //bp_error::general_not_authorized().await;
        }
    }
    Ok(())
}
