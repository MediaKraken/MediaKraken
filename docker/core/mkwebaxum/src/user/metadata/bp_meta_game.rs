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

#[path = "../../mk_lib_database_metadata_game.rs"]
mod mk_lib_database_metadata_game;

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_game.html")]
struct TemplateMetaGameContext<'a> {
    template_data: &'a Vec<mk_lib_database_metadata_game::DBMetaGameList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
}

pub async fn user_metadata_game(Extension(sqlx_pool): Extension<PgPool>, Path(page): Path<i32>) -> impl IntoResponse {
    let db_offset: i32 = (page * 30) - 30;
    let mut total_pages: i64 = mk_lib_database_metadata_game::mk_lib_database_metadata_game_count(
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
        "/user/metadata/game".to_string(),
    )
    .await
    .unwrap();
    let game_list = mk_lib_database_metadata_game::mk_lib_database_metadata_game_read(
        &sqlx_pool,
        String::new(),
        db_offset,
        30,
    )
    .await
    .unwrap();
    let template = TemplateMetaGameContext {
        template_data: &game_list,
        pagination_bar: &pagination_html,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_game_detail.html")]
struct TemplateMetaGameDetailContext {
    template_data: serde_json::Value,
}

pub async fn user_metadata_game_detail(Extension(sqlx_pool): Extension<PgPool>, Path(guid): Path<uuid::Uuid>) -> impl IntoResponse {
    let template = TemplateMetaGameDetailContext {
        template_data: json!({}),
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

/*
@blueprint_user_metadata_game.route('/user_meta_game', methods=["GET", "POST"])
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_game.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_game(request):
    """
    Display game list metadata
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    request.ctx.session['search_page'] = 'meta_game'
    db_connection = await request.app.db_pool.acquire()
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_meta_game',
                                                                      item_count=await request.app.db_functions.db_table_count(
                                                                          table_name='mm_metadata_game_software_info',
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    media_data = await request.app.db_functions.db_meta_game_list(offset,
                                                                  int(request.ctx.session[
                                                                          'per_page']),
                                                                  request.ctx.session[
                                                                      'search_text'],
                                                                  db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'media_game': media_data,
        'pagination_bar': pagination,
    }


@blueprint_user_metadata_game.route('/user_meta_game_detail/<guid>')
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_game_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_game_detail(request, guid):
    """
    Display game metadata detail
    """
    db_connection = await request.app.db_pool.acquire()
    media_data = await \
        request.app.db_functions.db_meta_game_by_guid(guid, db_connection=db_connection)[
            'gi_game_info_json']
    await request.app.db_pool.release(db_connection)
    return {
        'guid': guid,
        'data': media_data,
        'data_review': None,
    }

 */
