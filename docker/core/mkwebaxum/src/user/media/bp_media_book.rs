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

#[path = "../../mk_lib_database_media_book.rs"]
mod mk_lib_database_media_book;

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_book.html")]
struct TemplateMediaBookContext<'a> {
    template_data: &'a Vec<mk_lib_database_media_book::DBMediaBookList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
}

pub async fn user_media_book(
    Extension(sqlx_pool): Extension<PgPool>,
    Path(page): Path<i32>,
) -> impl IntoResponse {
    let db_offset: i32 = (page * 30) - 30;
    let mut total_pages: i64 =
        mk_lib_database_media_book::mk_lib_database_media_book_count(&sqlx_pool, String::new())
            .await
            .unwrap();
    if total_pages > 0 {
        total_pages = total_pages / 30;
    }
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
        total_pages,
        page,
        "/user/media/book".to_string(),
    )
    .await
    .unwrap();
    let book_list = mk_lib_database_media_book::mk_lib_database_media_book_read(
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
    let template = TemplateMediaBookContext {
        template_data: &book_list,
        template_data_exists: &template_data_exists,
        pagination_bar: &pagination_html,
        page: &page_usize,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_book_detail.html")]
struct TemplateMediaBookDetailContext {
    template_data: serde_json::Value,
}

pub async fn user_media_book_detail(
    Extension(sqlx_pool): Extension<PgPool>,
    Path(guid): Path<uuid::Uuid>,
) -> impl IntoResponse {
    //let tmp_uuid = sqlx::types::Uuid::parse_str(&guid.to_string()).unwrap();
    let template = TemplateMediaBookDetailContext {
        template_data: json!({}),
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

/*

@blueprint_user_periodical.route('/user_periodical', methods=['GET'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_periodical.html')
@common_global.auth.login_required
pub async fn url_bp_user_periodical_list(request):
    """
    Display periodical page
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    request.ctx.session['search_page'] = 'media_periodicals'
    db_connection = await request.app.db_pool.acquire()
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_meta_game',
                                                                      item_count=await request.app.db_functions.db_media_book_list_count(
                                                                          request.ctx.session[
                                                                              'search_text'],
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    media_data = await request.app.db_functions.db_media_book_list(offset,
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


@blueprint_user_periodical.route('/user_periodical_detail/<guid>', methods=['GET'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_periodical_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_periodical_detail(request, guid):
    """
    Display periodical detail page
    """
    return {}

 */
