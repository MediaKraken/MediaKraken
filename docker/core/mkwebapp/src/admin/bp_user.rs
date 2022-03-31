use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, tera::Tera, context};
use rocket_auth::{Users, Error, Auth, Signup, Login, AdminUser};
use uuid::Uuid;
use paginate::Pages;

#[get("/admin_user")]
pub async fn admin_user(sqlx_pool: &rocket::State<sqlx::PgPool>) -> Template {
    Template::render("bss_admin/bss_admin_user", context! {})
}

#[get("/admin_user_detail/<guid>")]
pub async fn admin_user_detail(sqlx_pool: &rocket::State<sqlx::PgPool>, guid: &str) -> Template {
    Template::render("bss_admin/bss_admin_user_detail", context! {})
}

/*
@blueprint_admin_users.route('/admin_user_delete', methods=["POST"])
@common_global.auth.login_required
async def url_bp_admin_user_delete(request):
    """
    Delete user action 'page'
    """
    db_connection = await request.app.db_pool.acquire()
    await request.app.db_functions.db_user_delete(request.form['id'], db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return json.dumps({'status': 'OK'})


@blueprint_admin_users.route("/admin_user_detail/<guid>")
@common_global.jinja_template.template('bss_admin/bss_admin_user_detail.html')
@common_global.auth.login_required
async def url_bp_admin_user_detail(request, guid):
    """
    Display user details
    """
    db_connection = await request.app.db_pool.acquire()
    data_user = await request.app.db_functions.db_user_detail(guid, db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {'data_user': data_user}


@blueprint_admin_users.route("/admin_users")
@common_global.jinja_template.template('bss_admin/bss_admin_user.html')
@common_global.auth.login_required
async def url_bp_admin_user(request):
    """
    Display user list
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    db_connection = await request.app.db_pool.acquire()
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/admin/admin_users',
                                                                      item_count=await request.app.db_functions.db_user_count(
                                                                          user_name=None,
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    data_users = await request.app.db_functions.db_user_list_name(offset,
                                                                  int(request.ctx.session[
                                                                          'per_page']),
                                                                  db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'users': data_users,
        'pagination_links': pagination,
        'page': page,
        'per_page': int(request.ctx.session['per_page'])
    }

 */