#![cfg_attr(debug_assertions, allow(dead_code))]

use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use axum_session_auth::*;
use axum_session_auth::{AuthConfig, AuthSession, AuthSessionLayer, Authentication};
use serde_json::json;
use sqlx::postgres::PgPool;
use stdext::function_name;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../../mk_lib_database_metadata_collection.rs"]
mod mk_lib_database_metadata_collection;

use crate::mk_lib_database_user;

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_movie_collection.html")]
struct TemplateMediaCollectionContext<'a> {
    template_data: &'a Vec<mk_lib_database_metadata_collection::DBMetaCollectionList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
}

pub async fn user_media_collection(
    Extension(sqlx_pool): Extension<PgPool>,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(page): Path<i64>,
) -> impl IntoResponse {
    let db_offset: i64 = (page * 30) - 30;
    let total_pages: i64 =
        mk_lib_database_metadata_collection::mk_lib_database_metadata_collection_count(
            &sqlx_pool,
            String::new(),
        )
        .await
        .unwrap();
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
        total_pages,
        page,
        "/user/media/collection".to_string(),
    )
    .await
    .unwrap();
    let collection_list =
        mk_lib_database_metadata_collection::mk_lib_database_metadata_collection_read(
            &sqlx_pool,
            String::new(),
            db_offset,
            30,
        )
        .await
        .unwrap();
    let mut template_data_exists = false;
    if collection_list.len() > 0 {
        template_data_exists = true;
    }
    let page_usize = page as usize;
    let template = TemplateMediaCollectionContext {
        template_data: &collection_list,
        template_data_exists: &template_data_exists,
        pagination_bar: &pagination_html,
        page: &page_usize,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_movie_collection_detail.html")]
struct TemplateMediaCollectionDetailContext {
    template_data: serde_json::Value,
}

pub async fn user_media_collection_detail(
    Extension(sqlx_pool): Extension<PgPool>,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(guid): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let template = TemplateMediaCollectionDetailContext {
        template_data: json!({}),
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

/*

@blueprint_user_media_collection.route('/user_media_movie_collection', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_movie_collection.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_movie_collection(request):
    """
    Display movie collection metadata
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    media = []
    db_connection = await request.app.db_pool.acquire()
    for row_data in await request.app.db_functions.db_collection_list(offset,
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      request.ctx.session[
                                                                          'search_text'],
                                                                      db_connection):
        if 'Poster' in row_data['mm_metadata_collection_imagelocal_json']:
            media.append((row_data['mm_metadata_collection_guid'],
                          row_data['mm_metadata_collection_name'],
                          row_data['mm_metadata_collection_imagelocal_json']['Poster']))
        else:
            media.append((row_data['mm_metadata_collection_guid'],
                          row_data['mm_metadata_collection_name'], None))
    request.ctx.session['search_page'] = 'meta_movie_collection'
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_media_movie_collection',
                                                                      item_count=await request.app.db_functions.db_collection_list_count(
                                                                          search_value=
                                                                          request.ctx.session[
                                                                              'search_text'],
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


@blueprint_user_media_collection.route('/user_media_movie_collection_detail/<guid>')
@common_global.jinja_template.template(
    'bss_user/metadata/bss_user_metadata_movie_collection_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_movie_collection_detail(request, guid):
    """
    Display movie collection metadata detail
    """
    db_connection = await request.app.db_pool.acquire()
    data_metadata = await request.app.db_functions.db_collection_read_by_guid(guid,
                                                                              db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    json_metadata = data_metadata['mm_metadata_collection_json']
    json_imagedata = data_metadata['mm_metadata_collection_imagelocal_json']
    # poster image
    try:
        if json_imagedata['Poster'] != None:
            data_poster_image = json_imagedata['Poster']
        else:
            data_poster_image = None
    except:
        data_poster_image = None
    # background image
    try:
        if json_imagedata['Backdrop'] != None:
            data_background_image = json_imagedata['Backdrop']
        else:
            data_background_image = None
    except:
        data_background_image = None
    return {
        'data_name': json_metadata['name'],
        'data_poster_image': data_poster_image,
        'data_background_image': data_background_image,
        'json_metadata': json_metadata,
    }

 */
