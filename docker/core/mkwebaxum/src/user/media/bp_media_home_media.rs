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

#[path = "../../mk_lib_database_media_home_media.rs"]
mod mk_lib_database_media_home_media;

use crate::mk_lib_database_user;

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_home_movie.html")]
struct TemplateMediaHomeContext<'a> {
    template_data: &'a Vec<mk_lib_database_media_home_media::DBMediaHomeMediaList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
}

pub async fn user_media_home_media(
    Extension(sqlx_pool): Extension<PgPool>,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(page): Path<i64>,
) -> impl IntoResponse {
    let db_offset: i64 = (page * 30) - 30;
    let total_pages: i64 =
        mk_lib_database_media_home_media::mk_lib_database_media_home_media_count(
            &sqlx_pool,
            String::new(),
        )
        .await
        .unwrap();
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
        total_pages,
        page,
        "/user/media/home_media".to_string(),
    )
    .await
    .unwrap();
    let home_list = mk_lib_database_media_home_media::mk_lib_database_media_home_media_read(
        &sqlx_pool,
        String::new(),
        db_offset,
        30,
    )
    .await
    .unwrap();
    let mut template_data_exists = false;
    if home_list.len() > 0 {
        template_data_exists = true;
    }
    let page_usize = page as usize;
    let template = TemplateMediaHomeContext {
        template_data: &home_list,
        template_data_exists: &template_data_exists,
        pagination_bar: &pagination_html,
        page: &page_usize,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_home_movie_detail.html")]
struct TemplateMediaHomeDetailContext {
    template_data: serde_json::Value,
}

pub async fn user_media_home_media_detail(
    Extension(sqlx_pool): Extension<PgPool>,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(guid): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let template = TemplateMediaHomeDetailContext {
        template_data: json!({}),
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

/*
@blueprint_user_home_media.route('/user_home_media', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_home_movie.html')
@common_global.auth.login_required
pub async fn url_bp_user_home_media_list(request):
    """
    Display home page for home media
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    db_connection = await request.app.db_pool.acquire()
    media_data = await request.app.db_functions.db_media_movie_list(
        class_guid=common_global.DLMediaType.Movie_Home.value,
        list_type=None,
        list_genre='All',
        list_limit=int(request.ctx.session[
                           'per_page']),
        group_collection=False,
        offset=offset,
        include_remote=False,
        search_text=request.ctx.session[
            'search_text'], db_connection=db_connection)
    # pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
    #                                                                   url='/user/user_home_media',
    #                                                                   item_count=await request.app.db_functions.db_meta_game_system_list_count(
    #                                                                       db_connection),
    #                                                                   client_items_per_page=
    #                                                                   int(request.ctx.session[
    #                                                                           'per_page']),
    #                                                                   format_number=True)
    await request.app.db_pool.release(db_connection)
    return {
        'media': media_data
    }


@blueprint_user_home_media.route('/user_home_media_detail/<guid>')
@common_global.jinja_template.template('bss_user/media/bss_user_media_home_movie_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_home_media_detail(request, guid):
    """
    Display home page for home media
    """
    return {}

 */
