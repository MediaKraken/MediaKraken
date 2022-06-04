use rocket::{get, post, form::Form, routes};
use rocket::Request;
use rocket_dyn_templates::{Template, tera::Tera};
use rocket_auth::{Users, Error, Auth, Signup, Login, User, AdminUser};
use serde::{Serialize, Deserialize};
use rocket::serde::json;
use rocket::{form::*, response::Redirect, State};
use serde_json::json;
use rocket::request::{self, FromRequest};

#[path = "../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../mk_lib_database_user.rs"]
mod mk_lib_database_user;

#[derive(Serialize)]
struct TemplateAdminUserContext<> {
    template_data: Vec<mk_lib_database_user::DBUserList>,
    pagination_bar: String,
}

#[get("/admin_user?<page>")]
pub async fn admin_user(sqlx_pool: &rocket::State<sqlx::PgPool>, user: AdminUser, page: i8) -> Template {
    let total_pages: i32 = mk_lib_database_user::mk_lib_database_user_count(&sqlx_pool, String::new()).await.unwrap() / 30;
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(total_pages, page).await.unwrap();
    let user_list = mk_lib_database_user::mk_lib_database_user_read(&sqlx_pool, 0, 30).await.unwrap();
    Template::render("bss_admin/bss_admin_user", &TemplateAdminUserContext {
        template_data: user_list,
        pagination_bar: pagination_html,
    })
}

#[get("/admin_user_detail/<guid>")]
pub async fn admin_user_detail(sqlx_pool: &rocket::State<sqlx::PgPool>,
     user: AdminUser, guid: rocket::serde::uuid::Uuid) -> Template {
    Template::render("bss_admin/bss_admin_user_detail", tera::Context::new().into_json())
}

/*
@blueprint_admin_users.route('/admin_user_delete', methods=["POST"])
@common_global.auth.login_required
pub async fn url_bp_admin_user_delete(request):
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
pub async fn url_bp_admin_user_detail(request, guid):
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
pub async fn url_bp_admin_user(request):
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
        'pagination_bar': pagination,
        'page': page,
        'per_page': int(request.ctx.session['per_page'])
    }

 */