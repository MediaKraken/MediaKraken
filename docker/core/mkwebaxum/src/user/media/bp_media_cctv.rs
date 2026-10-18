use crate::AppState;
use crate::mk_lib_database;
use askama::Template;
use axum::{
    extract::State,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Debug)]
struct TemplateCctvCamera {
    template_camera_name: String,
    template_camera_feed: String,
}

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_cctv.html")]
struct TemplateCctvContext<'a> {
    template_data: &'a Vec<TemplateCctvCamera>,
    template_data_exists: &'a bool,
    page_title: Option<String>,
}

pub async fn user_media_cctv(
    _state: State<AppState>,
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
        let reply_html = template.render().unwrap();
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let camera_list: Vec<TemplateCctvCamera> = Vec::new();
        let mut template_data_exists = false;
        if !camera_list.is_empty() {
            template_data_exists = true;
        }
        let template = TemplateCctvContext {
            template_data: &camera_list,
            template_data_exists: &template_data_exists,
            page_title: Some("MediaKraken CCTV".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}
