use crate::mk_lib_database;
use askama::Template;
use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use axum_session::{SessionConfig, SessionLayer};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_cron.html")]
struct TemplateCronContext<'a> {
    template_data: &'a Vec<mk_lib_database::mk_lib_database_cron::DBCronList>,
    template_data_exists: &'a bool,
        page_title: Option<String>,

}

pub async fn admin_cron(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::GET],
        false,
    )
    .requires(Rights::any([Rights::permission("Admin::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        let template = TemplateError403Context {};
        let reply_html = template.render().unwrap();
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let cron_list =
            mk_lib_database::mk_lib_database_cron::mk_lib_database_cron_service_read(&sqlx_pool)
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
}

pub async fn admin_cron_run(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(guid): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::GET],
        false,
    )
    .requires(Rights::any([Rights::permission("Admin::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        Redirect::to("/error/403")
        // let template = TemplateError403Context {};
        // let reply_html = template.render().unwrap();
        // (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let row_data = mk_lib_database::mk_lib_database_cron::mk_lib_database_cron_service_json(
            &sqlx_pool, guid,
        )
        .await
        .unwrap();
        let (rabbit_connection, rabbit_channel) =
            mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkwebapp")
                .await
                .unwrap();
        let _result = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_publish(
            rabbit_channel.clone(),
            row_data["route_key"].as_str().unwrap(),
            row_data.to_string(),
        )
        .await
        .unwrap();
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_close(rabbit_channel, rabbit_connection)
            .await
            .unwrap();
        let _result = mk_lib_database::mk_lib_database_cron::mk_lib_database_cron_time_update(
            &sqlx_pool, guid,
        )
        .await
        .unwrap();
        Redirect::to("/admin/cron")
    }
}

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

 */
