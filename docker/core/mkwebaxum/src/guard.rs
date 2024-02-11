use crate::bp_error;
use crate::error_handling;
use axum::http::Method;
use axum::{
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response, Redirect},
    routing::get,
    Extension, Router,
};
use axum_session_auth::{Auth, AuthSession, Rights, SessionPgPool};
use mk_lib_database;
use serde_json::json;
use sqlx::PgPool;
use stdext::function_name;
use std::error::Error;

pub async fn guard_page_by_user(
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    user_admin: bool,
    //) -> Result<Response, error_handling::MKAxumError> {
) -> Result<bool, Box<dyn Error>> {
//) -> Redirect {
    println!("Guard start");
    let mut authed_user = true;
    let current_user = auth.current_user.clone().unwrap_or_default();
    println!("Guard user {:?}", current_user);
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
            // return Redirect::temporary("/error/403");
            authed_user = false;
            //return Err(error_handling::MKAxumError::Error403);
            //bp_error::general_not_administrator().await;
        }
    } else {
        if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
            [Method::GET],
            false,
        )
        .requires(Rights::any([
            //Rights::permission("Category::View"),
            Rights::permission("User::View"),
        ]))
        .validate(&current_user, &method, None)
        .await
        {
            println!("here I am 2");
            //return Redirect::temporary("/error/401");
            authed_user = false;
            //return Err(error_handling::MKAxumError::Error401);
            //bp_error::general_not_authorized().await;
        }
    }
    // let request: Request<()> = Request::default();
    // let uri = request.uri();
    // Redirect::temporary(uri.path())
    //println!("Guard end");
    //Ok(next.run(req).await)
    Ok(authed_user)
}
