use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};
use rocket::serde::{Serialize, Deserialize, json::Json};

#[path = "../../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../../mk_lib_database_media_music.rs"]
mod mk_lib_database_media_music;

#[derive(Serialize)]
struct TemplateMediaMusicContext<> {
    template_data: Vec<mk_lib_database_media_music::DBMediaMusicList>,
    pagination_bar: String,
}

#[get("/media/music?<page>")]
pub async fn user_media_music(sqlx_pool: &rocket::State<sqlx::PgPool>, user: User, page: i8) -> Template {
    let total_pages: i32 = mk_lib_database_media_music::mk_lib_database_media_music_count(&sqlx_pool, String::new()).await.unwrap() / 30;
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(total_pages, page).await.unwrap();
    let music_list = mk_lib_database_media_music::mk_lib_database_media_music_read(&sqlx_pool, String::new(), 0, 30).await.unwrap();
    Template::render("bss_user/media/bss_user_media_music_album", &TemplateMediaMusicContext {
        template_data: music_list,
        pagination_bar: pagination_html,
    })
}

#[get("/media/music_detail/<guid>")]
pub async fn user_media_music_detail(sqlx_pool: &rocket::State<sqlx::PgPool>,
     user: User, guid: rocket::serde::uuid::Uuid) -> Template {
        let tmp_uuid = sqlx::types::Uuid::parse_str(&guid.to_string()).unwrap();
        Template::render("bss_user/media/bss_user_media_music_album_detail", tera::Context::new().into_json())
}

/*

@blueprint_user_music.route("/user_album_detail/<guid>")
@common_global.jinja_template.template('bss_user/media/bss_user_media_music_album_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_album_detail(request, guid):
    """
    Display album detail page
    """
    return {}


@blueprint_user_music.route("/user_album_list")
@common_global.jinja_template.template('bss_user/media/bss_user_media_music_album.html')
@common_global.auth.login_required
pub async fn url_bp_user_album_list(request):
    """
    Display album page
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    media = []
    db_connection = await request.app.db_pool.acquire()
    for row_data in await request.app.db_functions.db_media_album_list(offset,
                                                                       int(request.ctx.session[
                                                                               'per_page']),
                                                                       request.ctx.session[
                                                                           'search_text'],
                                                                       db_connection=db_connection):
        if 'mm_metadata_album_json' in row_data:
            media.append((row_data['mm_metadata_album_guid'], row_data['mm_metadata_album_name'],
                          row_data['mm_metadata_album_json']))
        else:
            media.append((row_data['mm_metadata_album_guid'],
                          row_data['mm_metadata_album_name'], None))
    request.ctx.session['search_page'] = 'music_album'
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_album_list',
                                                                      item_count=await request.app.db_functions.db_media_album_count(
                                                                          request.ctx.session[
                                                                              'search_page'],
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    await request.app.db_pool.release(db_connection)
    return {'media': media,
            'pagination_bar': pagination,
            }

 */