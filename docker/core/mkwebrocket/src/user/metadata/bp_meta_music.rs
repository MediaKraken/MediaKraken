#![cfg_attr(debug_assertions, allow(dead_code))]

use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::Request;
use rocket_auth::{Auth, Error, Login, Signup, User, Users};
use rocket_dyn_templates::{tera::Tera, Template};
use serde_json::json;
use stdext::function_name;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../../mk_lib_database_metadata_music.rs"]
mod database::mk_lib_database_metadata_music;

#[derive(Serialize)]
struct TemplateMetaMusicContext {
    template_data: Vec<mk_lib_database_metadata_music::DBMetaMusicList>,
    pagination_bar: String,
}

#[get("/metadata/music/<page>")]
pub async fn user_metadata_music(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    user: User,
    page: i32,
) -> Template {
    let db_offset: i64 = (page * 30) - 30;
    let total_pages: i64 = database::mk_lib_database_metadata_music::mk_lib_database_metadata_music_count(
        &sqlx_pool,
        String::new(),
    )
    .await
    .unwrap();
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
        total_pages,
        page,
        "/user/metadata/music".to_string(),
    )
    .await
    .unwrap();
    let music_list = database::mk_lib_database_metadata_music::mk_lib_database_metadata_music_read(
        &sqlx_pool,
        String::new(),
        db_offset,
        30,
    )
    .await
    .unwrap();
    Template::render(
        "bss_user/metadata/bss_user_metadata_music_album",
        &TemplateMetaMusicContext {
            template_data: music_list,
            pagination_bar: pagination_html,
        },
    )
}

#[derive(Serialize)]
struct TemplateMetaMusicDetailContext {
    template_data: serde_json::Value,
}

#[get("/metadata/music_detail/<guid>")]
pub async fn user_metadata_music_detail(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    user: User,
    guid: rocket::serde::uuid::Uuid,
) -> Template {
    let tmp_uuid = sqlx::types::Uuid::parse_str(&guid.to_string()).unwrap();
    Template::render(
        "bss_user/metadata/bss_user_metadata_music_album_detail",
        tera::Context::new().into_json(),
    )
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
