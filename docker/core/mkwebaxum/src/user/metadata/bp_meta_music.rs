#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use stdext::function_name;
use serde_json::json;
use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use sqlx::postgres::PgPool;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../../mk_lib_database_metadata_music.rs"]
mod mk_lib_database_metadata_music;

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_music_album.html")]
struct TemplateMetaMusicContext<'a> {
    template_data: &'a Vec<mk_lib_database_metadata_music::DBMetaMusicList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
}

pub async fn user_metadata_music(Extension(sqlx_pool): Extension<PgPool>, Path(page): Path<i32>) -> impl IntoResponse {
    let db_offset: i32 = (page * 30) - 30;
    let mut total_pages: i64 =
        mk_lib_database_metadata_music::mk_lib_database_metadata_music_count(
            &sqlx_pool,
            String::new(),
        )
        .await
        .unwrap();
    if total_pages > 0 {
        total_pages = total_pages / 30;
    }
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
        total_pages,
        page,
        "/user/metadata/music".to_string(),
    )
    .await
    .unwrap();
    let music_list = mk_lib_database_metadata_music::mk_lib_database_metadata_music_read(
        &sqlx_pool,
        String::new(),
        db_offset,
        30,
    )
    .await
    .unwrap();
    let template = TemplateMetaMusicContext {
        template_data: &music_list,
        pagination_bar: &pagination_html,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_music_album_detail.html")]
struct TemplateMetaMusicDetailContext {
    template_data: serde_json::Value,
}

pub async fn user_metadata_music_detail(Extension(sqlx_pool): Extension<PgPool>, Path(guid): Path<uuid::Uuid>) -> impl IntoResponse {
    let template = TemplateMetaMusicDetailContext {
        template_data: json!({}),
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

/*
from common import common_global
from common import common_logging_elasticsearch_httpx
from common import common_pagination_bootstrap
from sanic import Blueprint

blueprint_user_metadata_music = Blueprint('name_blueprint_user_metadata_music', url_prefix='/user')


@blueprint_user_metadata_music.route('/user_meta_music_album_list', methods=['GET', 'POST'])
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
        if album_data['mmp_person_image'] != None:
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
