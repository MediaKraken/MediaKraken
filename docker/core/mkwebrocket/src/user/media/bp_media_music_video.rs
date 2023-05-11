use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::Request;
use rocket_auth::{Auth, Error, Login, Signup, User, Users};
use rocket_dyn_templates::{tera::Tera, Template};
use serde_json::json;
use stdext::function_name;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../../mk_lib_database_media_music_video.rs"]
mod database::mk_lib_database_media_music_video;

#[derive(Serialize)]
struct TemplateMediaMusicVideoContext {
    template_data: Vec<mk_lib_database_media_music_video::DBMediaMusicVideoList>,
    pagination_bar: String,
}

#[get("/media/music_video/<page>")]
pub async fn user_media_music_video(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    user: User,
    page: i32,
) -> Template {
    let db_offset: i64 = (page * 30) - 30;
    let total_pages: i64 =
        database::mk_lib_database_media_music_video::mk_lib_database_media_music_video_count(
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
        database::mk_lib_database_media_music_video::mk_lib_database_media_music_video_read(
            &sqlx_pool,
            String::new(),
            db_offset,
            30,
        )
        .await
        .unwrap();
    Template::render(
        "bss_user/media/bss_user_media_music_video",
        &TemplateMediaMusicVideoContext {
            template_data: music_video_list,
            pagination_bar: pagination_html,
        },
    )
}

#[derive(Serialize)]
struct TemplateMediaMusicVideoDetailContext {
    template_data: serde_json::Value,
}

#[get("/media/music_video_detail/<guid>")]
pub async fn user_media_music_video_detail(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    user: User,
    guid: rocket::serde::uuid::Uuid,
) -> Template {
    let tmp_uuid = sqlx::types::Uuid::parse_str(&guid.to_string()).unwrap();
    Template::render(
        "bss_user/media/bss_user_media_music_video_detail",
        tera::Context::new().into_json(),
    )
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
