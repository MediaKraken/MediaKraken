use askama::Template;
use axum::{
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    Extension,
};
use axum_session_auth::{AuthSession, SessionPgPool};
use mk_lib_database;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_user/bss_user_queue.html")]
struct UserQueueTemplate;

pub async fn user_queue(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let template = UserQueueTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
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
