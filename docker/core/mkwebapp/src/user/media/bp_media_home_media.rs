use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};
use uuid::Uuid;
use paginate::Pages;

#[path = "../../mk_lib_database_media_home_media.rs"]
mod mk_lib_database_media_home_media;

#[get("/media/home_media")]
pub async fn user_media_home_media() -> Template {
    Template::render("bss_user/media/bss_user_media_home_movie", context! {})
}

#[get("/media/home_media_detail/<guid>")]
pub async fn user_media_home_media_detail(guid: &str) -> Template {
    Template::render("bss_user/media/bss_user_media_home_movie_detail", context! {})
}


/*
@blueprint_user_home_media.route('/user_home_media', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_home_movie.html')
@common_global.auth.login_required
async def url_bp_user_home_media_list(request):
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
async def url_bp_user_home_media_detail(request, guid):
    """
    Display home page for home media
    """
    return {}

 */