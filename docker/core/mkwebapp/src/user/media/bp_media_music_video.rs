use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};
use uuid::Uuid;
use rocket::serde::{Serialize, Deserialize, json::Json};

#[path = "../../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../../mk_lib_database_media_music_video.rs"]
mod mk_lib_database_media_music_video;

#[derive(Serialize)]
struct TemplateMediaMusicVideoContext<> {
    template_data: Vec<mk_lib_database_media_music_video::DBMediaMusicVideoList>,
    pagination_bar: String,
}

#[get("/media/music_video?<page>")]
pub async fn user_media_music_video(sqlx_pool: &rocket::State<sqlx::PgPool>, user: User, page: i8) -> Template {
    let total_pages: i32 = mk_lib_database_media_music_video::mk_lib_database_media_music_video_count(&sqlx_pool, String::new()).await.unwrap() / 30;
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(total_pages, page).await.unwrap();
    let music_video_list = mk_lib_database_media_music_video::mk_lib_database_media_music_video_read(&sqlx_pool, String::new(), 0, 30).await.unwrap();
    Template::render("bss_user/media/bss_user_media_music_video", &TemplateMediaMusicVideoContext {
        template_data: music_video_list,
        pagination_bar: pagination_html,
    })
}

#[get("/media/music_video_detail/<guid>")]
pub async fn user_media_music_video_detail(sqlx_pool: &rocket::State<sqlx::PgPool>, user: User, guid: Uuid) -> Template {
    Template::render("bss_user/media/bss_user_media_music_video_detail", {})
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