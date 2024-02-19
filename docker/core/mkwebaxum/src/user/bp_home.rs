use askama::Template;
use axum::{
    http::{Method, Request, StatusCode},
    response::{Html, IntoResponse},
    Extension,
};
use axum_session_auth::{Auth, AuthSession, Rights, SessionPgPool};
use crate::mk_lib_database;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_user/bss_user_home.html")]
struct TemplateUserHomeContext<'a> {
    template_data_new_media: &'a bool,
    template_data_user_media_queue: &'a bool,
}

pub async fn user_home(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    // if !auth.is_authenticated() {   Can I simply do this?   As a signed in user.....has access unless guest
    let current_user = auth.current_user.clone().unwrap_or_default();
    println!("Current_user: {:?}", current_user);
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::GET],
        false,
    )
    .requires(Rights::any([Rights::permission("User::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        let template = TemplateError401Context {};
        let reply_html = template.render().unwrap();
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let mut new_media = false;
        if mk_lib_database::database_media::mk_lib_database_media::mk_lib_database_media_new_count(
            &sqlx_pool, 7,
        )
        .await
        .unwrap()
            > 0
        {
            new_media = true;
        }
        let template = TemplateUserHomeContext {
            template_data_new_media: &new_media,
            template_data_user_media_queue: &true,
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}
