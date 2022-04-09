use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};
use uuid::Uuid;
use paginator::{Paginator, PageItem};
use core::fmt::Write;

#[path = "../../mk_lib_database_metadata_game.rs"]
mod mk_lib_database_metadata_game;

#[get("/metadata/game")]
pub async fn user_metadata_game(sqlx_pool: &rocket::State<sqlx::PgPool>) -> Template {
    let game_list = mk_lib_database_metadata_game::mk_lib_database_metadata_game_read(&sqlx_pool, "".to_string(), 0, 30).await.unwrap();
    Template::render("bss_user/metadata/bss_user_metadata_game", context! {
        media_game: game_list,
    })
}

#[get("/metadata/game_detail/<guid>")]
pub async fn user_metadata_game_detail(sqlx_pool: &rocket::State<sqlx::PgPool>, guid: &str) -> Template {
    Template::render("bss_user/metadata/bss_user_metadata_game_detail", context! {})
}

/*
@blueprint_user_metadata_game.route('/user_meta_game', methods=["GET", "POST"])
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_game.html')
@common_global.auth.login_required
async def url_bp_user_metadata_game(request):
    """
    Display game list metadata
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    request.ctx.session['search_page'] = 'meta_game'
    db_connection = await request.app.db_pool.acquire()
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_meta_game',
                                                                      item_count=await request.app.db_functions.db_table_count(
                                                                          table_name='mm_metadata_game_software_info',
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    media_data = await request.app.db_functions.db_meta_game_list(offset,
                                                                  int(request.ctx.session[
                                                                          'per_page']),
                                                                  request.ctx.session[
                                                                      'search_text'],
                                                                  db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'media_game': media_data,
        'pagination_links': pagination,
    }


@blueprint_user_metadata_game.route('/user_meta_game_detail/<guid>')
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_game_detail.html')
@common_global.auth.login_required
async def url_bp_user_metadata_game_detail(request, guid):
    """
    Display game metadata detail
    """
    db_connection = await request.app.db_pool.acquire()
    media_data = await \
        request.app.db_functions.db_meta_game_by_guid(guid, db_connection=db_connection)[
            'gi_game_info_json']
    await request.app.db_pool.release(db_connection)
    return {
        'guid': guid,
        'data': media_data,
        'data_review': None,
    }

 */