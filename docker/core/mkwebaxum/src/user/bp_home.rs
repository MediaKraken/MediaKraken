use crate::AppState;
use crate::mk_lib_database;
use askama::Template;
use axum::extract::State;
use axum::{
    Extension,
    http::{Method, Request, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session::{SessionConfig, SessionLayer};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_user/bss_user_home.html")]
struct TemplateUserHomeContext<'a> {
    template_data_new_media: &'a bool,
    template_data_user_media_queue: &'a bool,
    page_title: Option<String>,
}

pub async fn user_home(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::GET],
        false,
    )
    .requires(Rights::any([Rights::permission("User::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        let template = TemplateError401Context {};
        let reply_html = template.render().map_err(|e| e.to_string())?;
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let mut new_media = false;
        if mk_lib_database::database_media::mk_lib_database_media::mk_lib_database_media_new_count(
            &state.sqlx_pool_ro,
            7,
        )
        .await
        .unwrap_or(0)
            > 0
        {
            new_media = true;
        }
        let template = TemplateUserHomeContext {
            template_data_new_media: &new_media,
            template_data_user_media_queue: &true,
            page_title: Some("MediaKraken".to_string()),
        };
        let reply_html = template.render().map_err(|e| e.to_string())?;
        (StatusCode::OK, Html(reply_html).into_response())
    }
}
