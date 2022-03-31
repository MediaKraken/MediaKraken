use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login, User};
use uuid::Uuid;
use paginate::Pages;

#[path = "../../mk_lib_database_metadata_sports.rs"]
mod mk_lib_database_metadata_sports;

#[get("/metadata/sports")]
pub async fn user_metadata_sports() -> Template {
    Template::render("bss_user/metadata/bss_user_metadata_sports", context! {})
}

#[get("/metadata/sports_detail/<guid>")]
pub async fn user_metadata_sports_detail(guid: &str) -> Template {
    Template::render("bss_user/metadata/bss_user_metadata_sports_detail", context! {})
}

/*
@blueprint_user_metadata_sports.route('/user_meta_sports_detail/<guid>')
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_sports_detail.html')
@common_global.auth.login_required
async def url_bp_user_metadata_sports_detail(request, guid):
    """
    Display sports detail metadata
    """
    db_connection = await request.app.db_pool.acquire()
    media_data = await request.app.db_functions.db_meta_sports_guid_by_thesportsdb(guid,
                                                                                   db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'guid': guid,
        'data': media_data
    }


@blueprint_user_metadata_sports.route('/user_meta_sports_list', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_sports.html')
@common_global.auth.login_required
async def url_bp_user_metadata_sports_list(request):
    """
    Display sports metadata list
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    media = []
    db_connection = await request.app.db_pool.acquire()
    for row_data in await request.app.db_functions.db_meta_sports_list(offset,
                                                                       int(request.ctx.session[
                                                                               'per_page']),
                                                                       request.ctx.session[
                                                                           'search_text'],
                                                                       db_connection=db_connection):
        media.append((row_data['mm_metadata_sports_guid'],
                      row_data['mm_metadata_sports_name']))
    request.ctx.session['search_page'] = 'meta_sports'
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_meta_sports_list',
                                                                      item_count=await request.app.db_functions.db_meta_sports_list_count(
                                                                          request.ctx.session[
                                                                              'search_text'],
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    media_data = await request.app.db_functions.db_meta_sports_list(offset,
                                                                    int(request.ctx.session[
                                                                            'per_page']),
                                                                    request.ctx.session[
                                                                        'search_text'],
                                                                    db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'media_sports_list': media_data,
        'pagination_links': pagination,
    }

 */