use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login};
use uuid::Uuid;

#[get("/media/game_servers")]
pub fn user_media_game_servers(user: User) -> Template {
    Template::render("bss_user/media/bss_user_media_game_server", context! {})
}

#[get("/media/game_servers_detail/<guid>")]
pub fn user_media_game_servers_detail(user: User, guid: uuid::Uuid) -> Template {
    Template::render("bss_user/media/bss_user_media_game_server_detail", context! {})
}

/*
@blueprint_user_game_servers.route('/user_game_server', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_game_server.html')
@common_global.auth.login_required
async def url_bp_user_game_server_list(request):
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
        'pagination_links': pagination,
    }

 */