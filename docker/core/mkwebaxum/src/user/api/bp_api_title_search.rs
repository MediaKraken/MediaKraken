use crate::mk_lib_database;
use askama::Template;
use axum::{
    http::{Method, Request, StatusCode},
    response::{Html, IntoResponse},
    Extension,
};
use axum_session_auth::{Auth, AuthSession, Rights, SessionPgPool};
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_api/bss_api_title_search.html")]
struct TemplateAPITitleSearchContext<'a> {
    template_data_movie_match: &'a Vec<TemplateMetaMovieList>,
    template_data_tv_match: &'a Vec<TemplateMetaTvList>,
    template_data_music_match: &'a Vec<TemplateMetaMusicList>,
}

pub async fn api_title_search(
    Extension(sqlx_pool): Extension<PgPool>,
    Path(title): Path<String>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let movie_metadata = mk_lib_database::database_metadata::mk_lib_database_metadata::mk_lib_database_metadata_movie_read(
        &sqlx_pool, title,
    )
    .await
    .unwrap();
    let tv_metadata = mk_lib_database::database_metadata::mk_lib_database_metadata::mk_lib_database_metadata_tv_read(
        &sqlx_pool, title,
)
.await
.unwrap();
    let music_metadata = mk_lib_database::database_metadata::mk_lib_database_metadata::mk_lib_database_metadata_music_read(
        &sqlx_pool, title,
)
.await
.unwrap();
    let template = TemplateAPITitleSearchContext {
        template_data_movie_match: &movie_metadata,
        template_data_tv_match: &tv_metadata,
        template_data_music_match: &music_metadata,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
