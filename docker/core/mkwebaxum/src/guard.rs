use crate::bp_error;
use crate::error_handling;
use axum::http::Method;
use axum::{
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Extension, Router,
};
use axum_session_auth::{Auth, AuthSession, Rights, SessionPgPool};
use mk_lib_database;
use serde_json::json;
use sqlx::PgPool;
use stdext::function_name;

// pub async fn auth<B>(
//     req: Request<B>,
//     next: Next<B>,
//     method: Method,
//     //auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
//     user_admin: bool,
// ) -> Result<Response, StatusCode> {
//     println!("in auth");
//     Ok(next.run(req).await)
// }

pub async fn guard_page_by_user(
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    user_admin: bool,
    //) -> Result<Response, error_handling::MKAxumError> {
) -> Result<(), StatusCode> {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if user_admin == true {
        if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
            [Method::GET],
            false,
        )
        .requires(Rights::any([Rights::permission("Admin::View")]))
        .validate(&current_user, &method, None)
        .await
        {
            println!("here I am");
            //return Err(error_handling::MKAxumError::Error403);
            //bp_error::general_not_administrator().await;
        }
    } else {
        if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
            [Method::GET],
            false,
        )
        .requires(Rights::any([
            Rights::permission("Category::View"),
            Rights::permission("User::View"),
        ]))
        .validate(&current_user, &method, None)
        .await
        {
            println!("here I am 2");
            //return Err(error_handling::MKAxumError::Error401);
            //bp_error::general_not_authorized().await;
        }
    }
    Ok(())
    //Ok(next.run(req).await)
}
