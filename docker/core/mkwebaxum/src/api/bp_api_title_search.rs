use crate::AppState;
use crate::axum_custom_filters::filters;
use crate::mk_lib_database;
use askama::Template;
use axum::extract::State;
use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_error/bss_error_500.html")]
struct TemplateError500Context {}

#[derive(Template)]
#[template(path = "bss_api/bss_api_title_search.html")]
struct TemplateAPITitleSearchContext<'a> {
    template_data_movie_match: &'a Vec<
        mk_lib_database::database_metadata::mk_lib_database_metadata_movie::DBMetaMovieList,
    >,
    template_data_tv_match:
        &'a Vec<mk_lib_database::database_metadata::mk_lib_database_metadata_tv::DBMetaTVShowList>,
    template_data_music_match: &'a Vec<
        mk_lib_database::database_metadata::mk_lib_database_metadata_music::DBMetaMusicList,
    >,
}

pub async fn api_title_search(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(title): Path<String>,
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
        let normalized_title = title.replace("%20", " ");
        let movie_title = normalized_title.clone();
        let tv_title = normalized_title.clone();
        let music_title = normalized_title;

        let (movie_metadata, tv_metadata, music_metadata) = match tokio::try_join!(
            mk_lib_database::database_metadata::mk_lib_database_metadata_movie::mk_lib_database_metadata_movie_read(
                &state.sqlx_pool_ro,
                movie_title,
                current_user.id,
                0,
                100,
            ),
            mk_lib_database::database_metadata::mk_lib_database_metadata_tv::mk_lib_database_metadata_tv_read(
                &state.sqlx_pool_ro,
                tv_title,
                0,
                100,
            ),
            mk_lib_database::database_metadata::mk_lib_database_metadata_music::mk_lib_database_metadata_music_read(
                &state.sqlx_pool_ro,
                music_title,
                0,
                100,
            )
        ) {
            Ok(results) => results,
            Err(_) => {
                let template = TemplateError500Context {};
                let reply_html = template.render().unwrap();
                return (StatusCode::INTERNAL_SERVER_ERROR, Html(reply_html).into_response());
            },
        };
        let template = TemplateAPITitleSearchContext {
            template_data_movie_match: &movie_metadata,
            template_data_tv_match: &tv_metadata,
            template_data_music_match: &music_metadata,
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}
