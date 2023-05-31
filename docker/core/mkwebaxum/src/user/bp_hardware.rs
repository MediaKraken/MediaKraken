use askama::Template;
use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    Extension,
};
use axum_session_auth::{AuthSession, SessionPgPool};
use mk_lib_database;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_user/hardware/bss_user_hardware.html")]
struct TemplateUserHardwareContext<'a> {
    template_data_phue_exists: &'a bool,
}

pub async fn user_hardware(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let mut phue_exists: bool = true;
    let template = TemplateUserHardwareContext {
        template_data_phue_exists: &phue_exists,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

#[derive(Template)]
#[template(path = "bss_user/hardware/bss_user_hardware_phue.html")]
struct TemplateUserHardwarePhueContext {
    template_data_phue: i32,
}

pub async fn user_hardware_phue(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let template = TemplateUserHardwarePhueContext {
        template_data_phue: 0,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

/*
@blueprint_user_hardware.route('/user_hardware', methods=['GET'])
@common_global.jinja_template.template('bss_user/hardware/bss_user_hardware.html')
@common_global.auth.login_required
pub async fn url_bp_user_hardware(request):
    """
    Display hardware page
    """
    db_connection = await request.app.db_pool.acquire()
    phue_hardware = await request.app.db_functions.db_hardware_device_count(
        hardware_manufacturer='Phue', db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'phue': phue_hardware
    }
 */
