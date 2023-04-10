#![cfg_attr(debug_assertions, allow(dead_code))]

use askama::Template;
use axum::{
    extract::Path,
    http::{header, HeaderMap, Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use axum_session_auth::*;
use axum_session_auth::{AuthConfig, AuthSession, AuthSessionLayer, Authentication};
use serde_json::json;
use sqlx::postgres::PgPool;
use stdext::function_name;

#[path = "../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../mk_lib_database_cron.rs"]
mod mk_lib_database_cron;

#[path = "../mk_lib_database_user.rs"]
mod mk_lib_database_user;

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_cron.html")]
struct TemplateCronContext<'a> {
    template_data: &'a Vec<mk_lib_database_cron::DBCronList>,
    template_data_exists: &'a bool,
}

pub async fn admin_cron(
    Extension(sqlx_pool): Extension<PgPool>,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let cron_list = mk_lib_database_cron::mk_lib_database_cron_service_read(&sqlx_pool)
        .await
        .unwrap();
    let mut cron_data: bool = false;
    if cron_list.len() > 0 {
        cron_data = true;
    }
    let template = TemplateCronContext {
        template_data: &cron_list,
        template_data_exists: &cron_data,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

// #[post("/cron_delete/<guid>")]
// pub async fn admin_cron_delete(sqlx_pool: &rocket::State<sqlx::PgPool>, user: AdminUser, guid: rocket::serde::uuid::Uuid) -> Template {
//     mk_lib_database_cron::mk_lib_database_cron_delete(&sqlx_pool, guid).await.unwrap();
// }

/*
@blueprint_admin_cron.route('/admin_cron_edit/<guid>', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_admin/bss_admin_cron_edit.html')
@common_global.auth.login_required
pub async fn url_bp_admin_cron_edit(request, guid):
    """
    Edit cron job page
    """
    form = BSSCronEditForm(request, csrf_enabled=False)
    if request.method == 'POST':
        if form.validate_on_submit():
            request.form['name']
            request.form['description']
            request.form['enabled']
            request.form['interval']
            request.form['time']
            request.form['json']
    return {
        'guid': guid, 'form': form
    }


@blueprint_admin_cron.route('/cron_run/<guid>', methods=['GET', 'POST'])
@common_global.auth.login_required(user_keyword='user')
pub async fn url_bp_admin_cron_run(request, user, guid):
    """
    Run cron jobs
    """
    await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                     message_text={
                                                                         'admin cron run': guid})
    db_connection = await request.app.db_pool.acquire()
    cron_job_data = await request.app.db_functions.db_cron_info(guid, db_connection)
    cron_json_data = cron_job_data['mm_cron_json']
    # submit the message
    common_network_pika.com_net_pika_send({'Type': cron_json_data['Type'],
                                           'User': user.id,
                                           'JSON': cron_json_data},
                                          exchange_name=cron_json_data[
                                              'exchange_key'],
                                          route_key=cron_json_data['route_key'])
    await request.app.db_functions.db_cron_time_update(cron_job_data['mm_cron_name'],
                                                       db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return redirect(request.app.url_for('name_blueprint_admin_cron.url_bp_admin_cron'))

 */
