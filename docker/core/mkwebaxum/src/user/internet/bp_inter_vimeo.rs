use askama::Template;
use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    Extension,
};
use axum_session_auth::{AuthSession, SessionPgPool};
use mk_lib_database;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_user/internet/bss_user_internet_vimeo.html")]
struct UserInternetVimeoTemplate<'a> {
    template_data: &'a Vec<mk_lib_database::mk_lib_database_cron::DBCronList>,
    template_data_exists: &'a bool,
}

pub async fn user_inter_vimeo(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let template = UserInternetVimeoTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_user/internet/bss_user_internet_vimeo_detail.html")]
struct UserInternetVimeoDetailTemplate;

pub async fn user_inter_vimeo_detail(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let template = UserInternetVimeoDetailTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
