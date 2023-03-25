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

mod filters {
    pub fn space_to_html(s: &str) -> ::askama::Result<String> {
        Ok(s.replace(" ", "%20"))
    }
}

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../../mk_lib_database_media_music.rs"]
mod mk_lib_database_media_music;

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_music_album.html")]
struct TemplateMediaMusicContext<'a> {
    template_data: &'a Vec<mk_lib_database_media_music::DBMediaMusicList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
}

pub async fn user_media_music(Extension(sqlx_pool): Extension<PgPool>, Path(page): Path<i32>) -> impl IntoResponse {
    let db_offset: i32 = (page * 30) - 30;
    let mut total_pages: i64 =
        mk_lib_database_media_music::mk_lib_database_media_music_count(&sqlx_pool, String::new())
            .await
            .unwrap();
    if total_pages > 0 {
        total_pages = total_pages / 30;
    }
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
        total_pages,
        page,
        "/user/media/music/".to_string(),
    )
    .await
    .unwrap();
    let music_list = mk_lib_database_media_music::mk_lib_database_media_music_read(
        &sqlx_pool,
        String::new(),
        db_offset,
        30,
    )
    .await
    .unwrap();
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
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_music_album_detail.html")]
struct TemplateMediaMusicDetailContext {
    template_data: serde_json::Value,
}

pub async fn user_media_music_detail(Extension(sqlx_pool): Extension<PgPool>, Path(guid): Path<uuid::Uuid>) -> impl IntoResponse {
    let template = TemplateMediaMusicDetailContext {
        template_data: json!({}),
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
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
