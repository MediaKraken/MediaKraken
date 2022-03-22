use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};
use uuid::Uuid;

#[get("/media/music")]
pub fn user_media_music(user: User) -> Template {
    Template::render("bss_user/media/bss_user_media_music_album", context! {})
}

#[get("/media/music_detail/<guid>")]
pub fn user_media_music_detail(user: User, guid: &str) -> Template {
    Template::render("bss_user/media/bss_user_media_music_album_detail", context! {})
}

/*

@blueprint_user_music.route("/user_album_detail/<guid>")
@common_global.jinja_template.template('bss_user/media/bss_user_media_music_album_detail.html')
@common_global.auth.login_required
async def url_bp_user_album_detail(request, guid):
    """
    Display album detail page
    """
    return {}


@blueprint_user_music.route("/user_album_list")
@common_global.jinja_template.template('bss_user/media/bss_user_media_music_album.html')
@common_global.auth.login_required
async def url_bp_user_album_list(request):
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
            'pagination_links': pagination,
            }

 */