use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};
use uuid::Uuid;
use paginator::{Paginator, PageItem};
use core::fmt::Write;

#[path = "../../mk_lib_database_media_music_video.rs"]
mod mk_lib_database_media_music_video;

#[get("/media/music_video")]
pub async fn user_media_music_video(sqlx_pool: &rocket::State<sqlx::PgPool>) -> Template {
    Template::render("bss_user/media/bss_user_media_music_video", context! {})
}

#[get("/media/music_video_detail/<guid>")]
pub async fn user_media_music_video_detail(sqlx_pool: &rocket::State<sqlx::PgPool>, guid: &str) -> Template {
    Template::render("bss_user/media/bss_user_media_music_video_detail", context! {})
}

/*
@blueprint_user_music_video.route('/user_music_video', methods=['GET'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_music_video.html')
@common_global.auth.login_required
async def url_bp_user_music_video_list(request):
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
        'pagination_links': pagination,
    }


@blueprint_user_music_video.route('/user_music_video_detail/<guid>', methods=['GET'])
@common_global.auth.login_required
async def url_bp_user_music_video_detail(request, guid):
    """
    Display music video detail page
    """
    return {}

 */