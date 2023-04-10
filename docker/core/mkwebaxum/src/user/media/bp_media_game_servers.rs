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

#[path = "../../mk_lib_database_game_servers.rs"]
mod mk_lib_database_game_servers;

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_game_server.html")]
struct TemplateMediaGameServersContext<'a> {
    template_data: &'a Vec<mk_lib_database_game_servers::DBGameServerList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
}

pub async fn user_media_game_servers(
    Extension(sqlx_pool): Extension<PgPool>,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(page): Path<i64>,
) -> impl IntoResponse {
    let db_offset: i64 = (page * 30) - 30;
    let total_pages: i64 =
        mk_lib_database_game_servers::mk_lib_database_game_server_count(&sqlx_pool, String::new())
            .await
            .unwrap();
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
        total_pages,
        page,
        "/user/media/game_servers".to_string(),
    )
    .await
    .unwrap();
    let game_server_list = mk_lib_database_game_servers::mk_lib_database_game_server_read(
        &sqlx_pool,
        String::new(),
        db_offset,
        30,
    )
    .await
    .unwrap();
    let mut template_data_exists = false;
    if game_server_list.len() > 0 {
        template_data_exists = true;
    }
    let page_usize = page as usize;
    let template = TemplateMediaGameServersContext {
        template_data: &game_server_list,
        template_data_exists: &template_data_exists,
        pagination_bar: &pagination_html,
        page: &page_usize,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_game_server_detail.html")]
struct TemplateMediaGameServerDetailContext {
    template_data: serde_json::Value,
}

pub async fn user_media_game_servers_detail(
    Extension(sqlx_pool): Extension<PgPool>,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(guid): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let template = TemplateMediaGameServerDetailContext {
        template_data: json!({}),
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

/*
@blueprint_user_game_servers.route('/user_game_server', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_game_server.html')
@common_global.auth.login_required
pub async fn url_bp_user_game_server_list(request):
    """
    Display game server page
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    db_connection = await request.app.db_pool.acquire()
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_game_server',
                                                                      item_count=await request.app.db_functions.db_table_count(
                                                                          table_name='mm_game_dedicated_servers',
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    media_data = await request.app.db_functions.db_game_server_list(offset,
                                                                    int(request.ctx.session[
                                                                            'per_page']),
                                                                    db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'media': media_data,
        'pagination_bar': pagination,
    }

 */
