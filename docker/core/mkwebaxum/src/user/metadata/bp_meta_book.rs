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

use crate::mk_lib_logging;

#[path = "../../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

use crate::database::mk_lib_database_metadata_book;

use crate::database::mk_lib_database_user;

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_book.html")]
struct TemplateMetaBookContext<'a> {
    template_data: &'a Vec<mk_lib_database_metadata_book::DBMetaBookList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
}

pub async fn user_metadata_book(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(page): Path<i64>,
) -> impl IntoResponse {
    let db_offset: i64 = (page * 30) - 30;
    let total_pages: i64 = mk_lib_database_metadata_book::mk_lib_database_metadata_book_count(
        &sqlx_pool,
        String::new(),
    )
    .await
    .unwrap();
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
        total_pages,
        page,
        "/user/metadata/book".to_string(),
    )
    .await
    .unwrap();
    let book_list = mk_lib_database_metadata_book::mk_lib_database_metadata_book_read(
        &sqlx_pool,
        String::new(),
        db_offset,
        30,
    )
    .await
    .unwrap();
    let mut template_data_exists = false;
    if book_list.len() > 0 {
        template_data_exists = true;
    }
    let page_usize = page as usize;
    let template = TemplateMetaBookContext {
        template_data: &book_list,
        template_data_exists: &template_data_exists,
        pagination_bar: &pagination_html,
        page: &page_usize,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_book_detail.html")]
struct TemplateMetaBookDetailContext {
    template_data: serde_json::Value,
}

pub async fn user_metadata_book_detail(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(guid): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let detail_data =
        mk_lib_database_metadata_book::mk_lib_database_metadata_book_detail(&sqlx_pool, guid)
            .await
            .unwrap();
    let template = TemplateMetaBookDetailContext {
        template_data: detail_data,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
/*

@blueprint_user_metadata_periodical.route('/user_meta_periodical', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_periodical.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_periodical(request):
    """
    Display periodical list page
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    item_list = []
    db_connection = await request.app.db_pool.acquire()
    for item_data in await request.app.db_functions.db_meta_periodical_list(offset,
                                                                            int(request.ctx.session[
                                                                                    'per_page']),
                                                                            request.ctx.session[
                                                                                'search_text'],
                                                                            db_connection=db_connection):
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             'person data': item_data})
        item_image = "img/missing_icon.jpg"
        item_list.append((item_data['mm_metadata_book_guid'],
                          item_data['mm_metadata_book_name'], item_image))
    request.ctx.session['search_page'] = 'meta_periodical'
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_meta_periodical',
                                                                      item_count=await request.app.db_functions.db_meta_periodical_list_count(
                                                                          request.ctx.session[
                                                                              'search_text'],
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    await request.app.db_pool.release(db_connection)
    return {
        'media_person': item_list,
        'pagination_bar': pagination,
    }


@blueprint_user_metadata_periodical.route('/user_meta_periodical_detail/<guid>')
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_periodical_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_periodical_detail(request, guid):
    """
    Display periodical detail page
    """
    db_connection = await request.app.db_pool.acquire()
    json_metadata = await request.app.db_functions.db_meta_periodical_by_uuid(guid,
                                                                              db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    try:
        data_name = json_metadata['mm_metadata_book_json']['title']
    except KeyError:
        data_name = 'NA'
    try:
        data_isbn = common_isbn.com_isbn_mask(json_metadata['mm_metadata_book_json']['isbn10'])
    except KeyError:
        data_isbn = 'NA'
    try:
        data_overview = json_metadata['mm_metadata_book_json']['summary']
    except KeyError:
        data_overview = 'NA'
    try:
        data_author = json_metadata['mm_metadata_book_json']['author_data'][0]['name']
    except KeyError:
        data_author = 'NA'
    try:
        data_publisher = json_metadata['mm_metadata_book_json']['publisher_name']
    except KeyError:
        data_publisher = 'NA'
    try:
        data_pages = json_metadata['mm_metadata_book_json']['physical_description_text']
    except KeyError:
        data_pages = 'NA'
    return {
        'data_name': data_name,
        'data_isbn': data_isbn,
        'data_overview': data_overview,
        'data_author': data_author,
        'data_publisher': data_publisher,
        'data_pages': data_pages,
        'data_item_image': "img/missing_icon.jpg",
    }

 */
