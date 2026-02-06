use crate::axum_custom_filters::filters;
use crate::mk_lib_database;
use askama::Template;
use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    Extension,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::{FromRow, Row};

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
    Extension(ReadOnlyPool(sqlx_pool_ro)): Extension<ReadOnlyPool>,
    Path(title): Path<String>,
) -> impl IntoResponse {
    let title = title.replace("%20", " ");
    let movie_metadata = mk_lib_database::database_metadata::mk_lib_database_metadata_movie::mk_lib_database_metadata_movie_read(
        &sqlx_pool_ro, title.clone(), 0, 100
    )
    .await
    .unwrap();
    let tv_metadata = mk_lib_database::database_metadata::mk_lib_database_metadata_tv::mk_lib_database_metadata_tv_read(
        &sqlx_pool_ro, title.clone(), 0, 100
    )
    .await
    .unwrap();
    let music_metadata = mk_lib_database::database_metadata::mk_lib_database_metadata_music::mk_lib_database_metadata_music_read(
            &sqlx_pool_ro, title.clone(), 0, 100
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
