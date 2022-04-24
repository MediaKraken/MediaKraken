use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};
use uuid::Uuid;
use rocket::serde::{Serialize, Deserialize, json::Json};

#[path = "../../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../../mk_lib_database_metadata_music_video.rs"]
mod mk_lib_database_metadata_music_video;

#[derive(Serialize)]
struct TemplateMetaMusicVideoContext<> {
    template_data: Vec<mk_lib_database_metadata_music_video::DBMetaMusicVideoList>,
    pagination_bar: String,
}

#[get("/metadata/music_video?<page>")]
pub async fn user_metadata_music_video(sqlx_pool: &rocket::State<sqlx::PgPool>, user: User, page: i8) -> Template {
    let total_pages: i32 = mk_lib_database_metadata_music_video::mk_lib_database_metadata_music_video_count(&sqlx_pool, "".to_string(), 0).await.unwrap() / 30;
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(total_pages, page).await.unwrap();
    let music_video_list = mk_lib_database_metadata_music_video::mk_lib_database_metadata_music_video_read(&sqlx_pool, "".to_string(), 0, 30).await.unwrap();
    Template::render("bss_user/metadata/bss_user_metadata_music_video", &TemplateMetaMusicVideoContext {
        template_data: music_video_list,
        pagination_bar: pagination_html,
    })
}

#[get("/metadata/music_video_detail/<guid>")]
pub async fn user_metadata_music_video_detail(sqlx_pool: &rocket::State<sqlx::PgPool>, user: User, guid: Uuid) -> Template {
    Template::render("bss_user/metadata/bss_user_metadata_music_video_detail", {})
}

/*

@blueprint_user_metadata_music_video.route('/user_meta_music_video', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_music_video.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_music_video(request):
    """
    Display metadata music video
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    request.ctx.session['search_page'] = 'meta_music_video'
    db_connection = await request.app.db_pool.acquire()
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_meta_music_video',
                                                                      item_count=await request.app.db_functions.db_meta_music_video_count(
                                                                          None, request.ctx.session[
                                                                              'search_text'],
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    media_data = await request.app.db_functions.db_meta_music_video_list(offset,
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


@blueprint_user_metadata_music_video.route('/user_meta_music_video_detail/<guid>')
@common_global.jinja_template.template(
    'bss_user/metadata/bss_user_metadata_music_video_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_music_video_detail(request, guid):
    """
    Display metadata music video detail
    """
    db_connection = await request.app.db_pool.acquire()
    media_data = await request.app.db_functions.db_meta_music_video_detail_uuid(guid,
                                                                                db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'media': media_data
    }

 */