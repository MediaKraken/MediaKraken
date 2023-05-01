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

use crate::database::mk_lib_database_media_music_video;

use crate::database::mk_lib_database_user;

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_music_video.html")]
struct TemplateMediaMusicVideoContext<'a> {
    template_data: &'a Vec<mk_lib_database_media_music_video::DBMediaMusicVideoList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
}

pub async fn user_media_music_video(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(page): Path<i64>,
) -> impl IntoResponse {
    let db_offset: i64 = (page * 30) - 30;
    let total_pages: i64 =
        mk_lib_database_media_music_video::mk_lib_database_media_music_video_count(
            &sqlx_pool,
            String::new(),
        )
        .await
        .unwrap();
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
        total_pages,
        page,
        "/user/media/music_video".to_string(),
    )
    .await
    .unwrap();
    let music_video_list =
        mk_lib_database_media_music_video::mk_lib_database_media_music_video_read(
            &sqlx_pool,
            String::new(),
            db_offset,
            30,
        )
        .await
        .unwrap();
    let mut template_data_exists: bool = false;
    if music_video_list.len() > 0 {
        template_data_exists = true;
    }
    let page_usize = page as usize;
    let template = TemplateMediaMusicVideoContext {
        template_data: &music_video_list,
        template_data_exists: &template_data_exists,
        pagination_bar: &pagination_html,
        page: &page_usize,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_music_video_detail.html")]
struct TemplateMediaMusicVideoDetailContext {
    template_data: serde_json::Value,
}

pub async fn user_media_music_video_detail(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(guid): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let template = TemplateMediaMusicVideoDetailContext {
        template_data: json!({}),
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

/*
@blueprint_user_music_video.route('/user_music_video', methods=['GET'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_music_video.html')
@common_global.auth.login_required
pub async fn url_bp_user_music_video_list(request):
    """
    Display music video page
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    request.ctx.session['search_page'] = 'media_music_video'
    db_connection = await request.app.db_pool.acquire()
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_music_video',
                                                                      item_count=await request.app.db_functions.db_music_video_list_count(
                                                                          request.ctx.session[
                                                                              'search_text'],
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    media_data = await request.app.db_functions.db_music_video_list(offset,
                                                                    int(request.ctx.session[
                                                                            'per_page']),
                                                                    request.ctx.session[
                                                                        'search_text'],
                                                                    db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'media_person': media_data,
        'pagination_bar': pagination,
    }


@blueprint_user_music_video.route('/user_music_video_detail/<guid>', methods=['GET'])
@common_global.auth.login_required
pub async fn url_bp_user_music_video_detail(request, guid):
    """
    Display music video detail page
    """
    return {}

 */
