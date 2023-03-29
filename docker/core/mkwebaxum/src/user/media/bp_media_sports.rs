#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use serde_json::json;
use sqlx::postgres::PgPool;
use stdext::function_name;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../../mk_lib_database_media_sports.rs"]
mod mk_lib_database_media_sports;

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_sports.html")]
struct TemplateMediaSportsContext<'a> {
    template_data: &'a Vec<mk_lib_database_media_sports::DBMediaSportsList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
}

pub async fn user_media_sports(
    Extension(sqlx_pool): Extension<PgPool>,
    Path(page): Path<i64>,
) -> impl IntoResponse {
    let db_offset: i64 = (page * 30) - 30;
    let total_pages: i64 =
        mk_lib_database_media_sports::mk_lib_database_media_sports_count(&sqlx_pool, String::new())
            .await
            .unwrap();
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
        total_pages,
        page,
        "/user/media/sports".to_string(),
    )
    .await
    .unwrap();
    let sports_list = mk_lib_database_media_sports::mk_lib_database_media_sports_read(
        &sqlx_pool,
        String::new(),
        db_offset,
        30,
    )
    .await
    .unwrap();
    let mut template_data_exists = false;
    if sports_list.len() > 0 {
        template_data_exists = true;
    }
    let page_usize = page as usize;
    let template = TemplateMediaSportsContext {
        template_data: &sports_list,
        template_data_exists: &template_data_exists,
        pagination_bar: &pagination_html,
        page: &page_usize,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_sports_detail.html")]
struct TemplateMediaSportsDetailContext {
    template_data: serde_json::Value,
}

pub async fn user_media_sports_detail(
    Extension(sqlx_pool): Extension<PgPool>,
    Path(guid): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let template = TemplateMediaSportsDetailContext {
        template_data: json!({}),
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

/*
# list of spoting events
@blueprint_user_sports.route("/user_sports", methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_sports.html')
@common_global.auth.login_required
pub async fn url_bp_user_sports(request):
    """
    Display sporting events page
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    media = []
    db_connection = await request.app.db_pool.acquire()
    for row_data in await request.app.db_functions.db_media_sports_list(
            common_global.DLMediaType.Sports.value,
            offset,
            int(request.ctx.session[
                    'per_page']),
            request.ctx.session[
                'search_text'], db_connection=db_connection):
        media.append((row_data['mm_metadata_sports_guid'],
                      row_data['mm_metadata_sports_name']))
    request.ctx.session['search_page'] = 'media_sports'
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_sports',
                                                                      item_count=await request.app.db_functions.db_media_sports_list_count(
                                                                          request.ctx.session[
                                                                              'search_text'],
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    await request.app.db_pool.release(db_connection)
    return {'media': media,
            'pagination_bar': pagination,
            }


@blueprint_user_sports.route("/user_sports_detail/<guid>", methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_sports_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_sports_detail(request, guid):
    """
    Display sports detail page
    """
    # poster image
    db_connection = await request.app.db_pool.acquire()
    media_data = await request.app.db_functions.db_meta_thesportsdb_select_by_guid(guid,
                                                                                   db_connection=db_connection)
    try:
        if json_metadata['LocalImages']['Poster'] != None:
            data_poster_image = json_metadata['LocalImages']['Poster']
        else:
            data_poster_image = None
    except:
        data_poster_image = None
    # background image
    try:
        if json_metadata['LocalImages']['Backdrop'] != None:
            data_background_image = json_metadata['LocalImages']['Backdrop']
        else:
            data_background_image = None
    except:
        data_background_image = None
    await request.app.db_pool.release(db_connection)
    return {
        'data': media_data,
        'data_poster_image': data_poster_image,
        'data_background_image': data_background_image,
    }

 */
