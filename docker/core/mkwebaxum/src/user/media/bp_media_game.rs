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

#[path = "../../mk_lib_database_media_game.rs"]
mod mk_lib_database_media_game;

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_game.html")]
struct TemplateMediaGameContext {
    template_data: Vec<mk_lib_database_media_game::DBMediaGameList>,
    pagination_bar: String,
}

#[get("/media/game/<page>")]
pub async fn user_media_game(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    user: User,
    page: i32,
) -> Template {
    let db_offset: i32 = (page * 30) - 30;
    let mut total_pages: i64 =
        mk_lib_database_media_game::mk_lib_database_media_game_count(&sqlx_pool, String::new())
            .await
            .unwrap();
    if total_pages > 0 {
        total_pages = total_pages / 30;
    }
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
        total_pages,
        page,
        "/user/media/game".to_string(),
    )
    .await
    .unwrap();
    let game_list = mk_lib_database_media_game::mk_lib_database_media_game_read(
        &sqlx_pool,
        String::new(),
        db_offset,
        30,
    )
    .await
    .unwrap();
    Template::render(
        "bss_user/media/bss_user_media_game.html",
        &TemplateMediaGameContext {
            template_data: game_list,
            pagination_bar: pagination_html,
        },
    )
}

#[derive(Serialize)]
struct TemplateMediaGameDetailContext {
    template_data: serde_json::Value,
}

#[get("/media/game_detail/<guid>")]
pub async fn user_media_game_detail(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    user: User,
    guid: rocket::serde::uuid::Uuid,
) -> Template {
    let tmp_uuid = sqlx::types::Uuid::parse_str(&guid.to_string()).unwrap();
    Template::render(
        "bss_user/media/bss_user_media_game_detail.html",
        tera::Context::new().into_json(),
    )
}

/*
@blueprint_user_game.route('/user_game', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_game.html')
@common_global.auth.login_required
pub async fn url_bp_user_game(request):
    """
    Display game page
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    request.ctx.session['search_page'] = 'media_games'
    db_connection = await request.app.db_pool.acquire()
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_game',
                                                                      item_count=await request.app.db_functions.db_meta_game_system_list_count(
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    media_data = await request.app.db_functions.db_meta_game_system_list(offset,
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


@blueprint_user_game.route('/user_game_detail/<guid>', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_game_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_game_detail(request, guid):
    """
    Display game detail page
    """
    return {}

 */