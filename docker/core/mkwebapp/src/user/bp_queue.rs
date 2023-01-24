#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use rocket::response::Redirect;
use rocket::Request;
use rocket_auth::{Auth, Error, Login, Signup, User, Users};
use rocket_dyn_templates::{tera::Tera, Template};
use stdext::function_name;
use serde_json::json;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../mk_lib_database_user_queue.rs"]
mod mk_lib_database_user_queue;

#[get("/queue")]
pub async fn user_queue(sqlx_pool: &rocket::State<sqlx::PgPool>, user: User) -> Template {
    Template::render("bss_user/bss_user_queue", tera::Context::new().into_json())
}

/*
@blueprint_user_queue.route("/user_queue", methods=['GET'])
@common_global.jinja_template.template('bss_user/user_queue.html')
@common_global.auth.login_required()
pub async fn url_bp_user_queue(request):
    """
    Display queue page
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    // TODO union read all four.....then if first "group"....add header in the html
    request.ctx.session['search_page'] = 'user_media_queue'
    db_connection = await request.app.db_pool.acquire()
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_queue',
                                                                      item_count=await request.app.db_functions.db_meta_queue_list_count(
                                                                          common_global.auth.current_user(
                                                                              request)[0],
                                                                          request.ctx.session[
                                                                              'search_text'],
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    media_data = await request.app.db_functions.db_meta_queue_list(common_global.auth.current_user(
        request)[0],
                                                                   offset,
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

 */
