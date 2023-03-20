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

#[path = "../../mk_lib_database_metadata_sports.rs"]
mod mk_lib_database_metadata_sports;

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_sports.html")]
struct TemplateMetaSportsContext {
    template_data: Vec<mk_lib_database_metadata_sports::DBMetaSportsList>,
    pagination_bar: String,
}

#[get("/metadata/sports/<page>")]
pub async fn user_metadata_sports(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    user: User,
    page: i32,
) -> Template {
    let db_offset: i32 = (page * 30) - 30;
    let mut total_pages: i64 =
        mk_lib_database_metadata_sports::mk_lib_database_metadata_sports_count(
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
        "/user/metadata/sports".to_string(),
    )
    .await
    .unwrap();
    let sports_list = mk_lib_database_metadata_sports::mk_lib_database_metadata_sports_read(
        &sqlx_pool,
        String::new(),
        db_offset,
        30,
    )
    .await
    .unwrap();
    Template::render(
        "bss_user/metadata/bss_user_metadata_sports.html",
        &TemplateMetaSportsContext {
            template_data: sports_list,
            pagination_bar: pagination_html,
        },
    )
}

#[derive(Serialize)]
struct TemplateMetaSportsDetailContext {
    template_data: serde_json::Value,
}

#[get("/metadata/sports_detail/<guid>")]
pub async fn user_metadata_sports_detail(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    user: User,
    guid: rocket::serde::uuid::Uuid,
) -> Template {
    let tmp_uuid = sqlx::types::Uuid::parse_str(&guid.to_string()).unwrap();
    Template::render(
        "bss_user/metadata/bss_user_metadata_sports_detail.html",
        tera::Context::new().into_json(),
    )
}

/*
@blueprint_user_metadata_sports.route('/user_meta_sports_detail/<guid>')
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_sports_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_sports_detail(request, guid):
    """
    Display sports detail metadata
    """
    db_connection = await request.app.db_pool.acquire()
    media_data = await request.app.db_functions.db_meta_sports_guid_by_thesportsdb(guid,
                                                                                   db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'guid': guid,
        'data': media_data
    }


@blueprint_user_metadata_sports.route('/user_meta_sports_list', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_sports.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_sports_list(request):
    """
    Display sports metadata list
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    media = []
    db_connection = await request.app.db_pool.acquire()
    for row_data in await request.app.db_functions.db_meta_sports_list(offset,
                                                                       int(request.ctx.session[
                                                                               'per_page']),
                                                                       request.ctx.session[
                                                                           'search_text'],
                                                                       db_connection=db_connection):
        media.append((row_data['mm_metadata_sports_guid'],
                      row_data['mm_metadata_sports_name']))
    request.ctx.session['search_page'] = 'meta_sports'
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_meta_sports_list',
                                                                      item_count=await request.app.db_functions.db_meta_sports_list_count(
                                                                          request.ctx.session[
                                                                              'search_text'],
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    media_data = await request.app.db_functions.db_meta_sports_list(offset,
                                                                    int(request.ctx.session[
                                                                            'per_page']),
                                                                    request.ctx.session[
                                                                        'search_text'],
                                                                    db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'media_sports_list': media_data,
        'pagination_bar': pagination,
    }

 */
