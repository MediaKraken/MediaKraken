use crate::AppState;
use crate::mk_lib_database;
use crate::user_preferences;
use askama::Template;
use axum::extract::Query;
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
use serde::Deserialize;
use serde_json::json;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_music_album.html")]
struct TemplateMetaMusicContext<'a> {
    template_data: &'a Vec<
        mk_lib_database::database_metadata::mk_lib_database_metadata_music_brainz::DBMetaMusicList,
    >,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
    page_title: Option<String>,
    pub current: Option<String>,
    pub genre_filter: Option<String>,
    pub genre_filter_query: Option<String>,
    pub primary_language_filter: Option<String>,
    pub status_filter: Option<String>,
    pub base_path: String,
}

#[derive(Debug, Deserialize)]
pub struct FilterQuery {
    pub starts_with: Option<String>,
}

fn normalize_starts_with(raw: Option<&str>) -> Option<String> {
    let value = raw.map(str::trim).filter(|value| !value.is_empty())?;
    let first_char = value.chars().next()?;

    if first_char == '#' {
        return Some("#".to_string());
    }
    if first_char.is_ascii_alphanumeric() {
        return Some(first_char.to_ascii_uppercase().to_string());
    }
    Some("#".to_string())
}

pub async fn user_metadata_music(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(page): Path<i64>,
    Query(params): Query<FilterQuery>,
) -> impl IntoResponse {
    let starts_with = normalize_starts_with(params.starts_with.as_deref());
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
        mk_lib_database::database_metadata::mk_lib_database_metadata_music_brainz::mk_lib_database_metadata_music_album_count(
           &state.sqlx_pool_ro,
            starts_with.clone().unwrap_or_default(),
        )
        .await
        ?;
        let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
            total_pages,
            page,
            "/user/metadata/music".to_string(),
            starts_with.as_deref(),
            pagination_count,
        )
        .await
        ?;
        let music_list =
        mk_lib_database::database_metadata::mk_lib_database_metadata_music_brainz::mk_lib_database_metadata_music_album_read(
           &state.sqlx_pool_ro,
            starts_with.clone().unwrap_or_default(),
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
        let template = TemplateMetaMusicContext {
            template_data: &music_list,
            template_data_exists: &template_data_exists,
            pagination_bar: &pagination_html,
            page: &page_usize,
            page_title: Some("MediaKraken Metadata Music".to_string()),
            current: starts_with,
            genre_filter: None,
            genre_filter_query: None,
            primary_language_filter: None,
            status_filter: None,
            base_path: "/user/metadata/music".to_string(),
        };
        let reply_html = template.render().map_err(|e| e.to_string())?;
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_music_album_detail.html")]
struct TemplateMetaMusicDetailContext {
    template_data: serde_json::Value,
    page_title: Option<String>,
}

pub async fn user_metadata_music_detail(
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
        let template = TemplateMetaMusicDetailContext {
            template_data: json!({}),
            page_title: Some("MediaKraken Metadata Music Detail".to_string()),
        };
        let reply_html = template.render().map_err(|e| e.to_string())?;
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

/*

blueprint_user_metadata_music = Blueprint('name_blueprint_user_metadata_music', url_prefix='/user')blueprint_user_metadata_music.route('/user_meta_music_album_list', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_music_album.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_music_album_list(request):
    """
    Display metadata of album list
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    media = []
    db_connection = await request.app.db_pool.acquire()
    for album_data in await request.app.db_functions.db_meta_music_album_list(offset,
                                                                              int(
                                                                                  request.ctx.session[
                                                                                      'per_page']),
                                                                              request.ctx.session[
                                                                                  'search_text'],
                                                                              db_connection=db_connection):
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             'album_data': album_data,
                                                                             'id': album_data[
                                                                                 'mm_metadata_album_guid'],
                                                                             'name': album_data[
                                                                                 'mm_metadata_album_name'],
                                                                             'json': album_data[
                                                                                 'mm_metadata_album_json']})
        if album_data['mm_metadata_person_person_image'] != None:
            if 'musicbrainz' in album_data['mm_metadata_album_image']['Images']:
                try:
                    album_image = album_data['mm_metadata_album_image']['Images']['musicbrainz']
                except:
                    album_image = "img/music_album_missing.png"
            else:
                album_image = "img/music_album_missing.png"
        else:
            album_image = "img/music_album_missing.png"
            media.append(
                (album_data['mm_metadata_album_guid'], album_data['mm_metadata_album_name'],
                 album_image))
    request.ctx.session['search_page'] = 'meta_album'
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_meta_music_album_list',
                                                                      item_count=await request.app.db_functions.db_table_count(
                                                                          table_name='mm_metadata_album',
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    await request.app.db_pool.release(db_connection)
    return {
        'media': media,
        'pagination_bar': pagination,
    }


@blueprint_user_metadata_music.route('/user_meta_music_album_song_list', methods=['GET', 'POST'])
@common_global.jinja_template.template(
    'bss_user/metadata/bss_user_metadata_music_album_detail.html')
@common_global.auth.login_required
pub async fn metadata_music_album_song_list(request):
    """
    Display metadata music song list
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    request.ctx.session['search_page'] = 'meta_music_song'
    db_connection = await request.app.db_pool.acquire()
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_meta_music_album_song_list',
                                                                      item_count=await request.app.db_functions.db_table_count(
                                                                          table_name='mm_metadata_music',
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    media_data = await request.app.db_functions.db_meta_music_song_list(offset,
                                                                        int(request.ctx.session[
                                                                                'per_page']),
                                                                        request.ctx.session[
                                                                            'search_text'],
                                                                        db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'media': media_data,
        'pagination_bar': pagination,
    }

 */
