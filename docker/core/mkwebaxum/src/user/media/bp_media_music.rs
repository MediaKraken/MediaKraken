use crate::AppState;
use crate::axum_custom_filters::filters;
use crate::mk_lib_database;
use crate::user_preferences;
use askama::Template;
use axum::extract::State;
use axum::response::Redirect;
use axum::{
    Extension,
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session::{SessionConfig, SessionLayer};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use mk_lib_common::mk_lib_common_pagination;
use serde_json::json;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_music_album.html")]
struct TemplateMediaMusicContext<'a> {
    template_data:
        &'a Vec<mk_lib_database::database_media::mk_lib_database_media_music::DBMediaMusicList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
    page_title: Option<String>,
}

pub async fn user_media_music(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(page): Path<i64>,
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
        let pagination_count =
            user_preferences::load_user_pagination_count(&state.sqlx_pool_ro, current_user.id)
                .await
                .unwrap_or(user_preferences::DEFAULT_PAGINATION_COUNT);
        let db_offset: i64 = (page * pagination_count) - pagination_count;
        let total_pages: i64 =
        mk_lib_database::database_media::mk_lib_database_media_music::mk_lib_database_media_music_count(
            &state.sqlx_pool_ro,
            String::new(),
        )
        .await
        ?;
        let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
            total_pages,
            page,
            "/user/media/music/".to_string(),
            None,
            pagination_count,
        )
        .await
        ?;
        let music_list =
        mk_lib_database::database_media::mk_lib_database_media_music::mk_lib_database_media_music_read(
           &state.sqlx_pool_ro,
            String::new(),
            db_offset,
            pagination_count,
        )
        .await
        ?;
        let mut template_data_exists = false;
        if music_list.len() > 0 {
            template_data_exists = true;
        }
        let page_usize = page as usize;
        let template = TemplateMediaMusicContext {
            template_data: &music_list,
            template_data_exists: &template_data_exists,
            pagination_bar: &pagination_html,
            page: &page_usize,
            page_title: Some("MediaKraken Music".to_string()),
        };
        let reply_html = template.render().map_err(|e| e.to_string())?;
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_music_album_detail.html")]
struct TemplateMediaMusicDetailContext {
    template_data: serde_json::Value,
    template_data_exists: bool,
    page_title: Option<String>,
}

pub async fn user_media_music_detail(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(guid): Path<uuid::Uuid>,
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
        let template = TemplateMediaMusicDetailContext {
            template_data: json!({}),
            template_data_exists: false,
            page_title: Some("MediaKraken Music Detail".to_string()),
        };
        let reply_html = template.render().map_err(|e| e.to_string())?;
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

/*

@blueprint_user_music.route("/user_album_list")
@common_global.jinja_template.template('bss_user/media/bss_user_media_music_album.html')
@common_global.auth.login_required
pub async fn url_bp_user_album_list(request):
    """
    Display album page
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    media = []
    db_connection = await request.app.db_pool.acquire()
    for row_data in await request.app.db_functions.db_media_album_list(offset,
                                                                       int(request.ctx.session[
                                                                               'per_page']),
                                                                       request.ctx.session[
                                                                           'search_text'],
                                                                       db_connection=db_connection):
        if 'mm_metadata_album_json' in row_data:
            media.append((row_data['mm_metadata_album_guid'], row_data['mm_metadata_album_name'],
                          row_data['mm_metadata_album_json']))
        else:
            media.append((row_data['mm_metadata_album_guid'],
                          row_data['mm_metadata_album_name'], None))
    request.ctx.session['search_page'] = 'music_album'
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_album_list',
                                                                      item_count=await request.app.db_functions.db_media_album_count(
                                                                          request.ctx.session[
                                                                              'search_page'],
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    await request.app.db_pool.release(db_connection)
    return {'media': media,
            'pagination_bar': pagination,
            }

 */
