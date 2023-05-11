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
use mk_lib_database;
use mk_lib_logging::mk_lib_logging;
use serde_json::json;
use sqlx::postgres::PgPool;
use stdext::function_name;

#[derive(Template)]
#[template(path = "bss_user/bss_user_home.html")]
struct TemplateUserHomeContext<'a> {
    template_data_new_media: &'a bool,
    template_data_user_media_queue: &'a bool,
}

pub async fn user_home(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let template = TemplateUserHomeContext {
        template_data_new_media: &true,
        template_data_user_media_queue: &true,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

/*
@blueprint_user_homepage.route('/user_home', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/bss_user_home.html')
@common_global.auth.login_required
pub async fn url_bp_user_homepage(request):
    """
    Display user home page
    """
    print('current user - url_bp_user_homepage', common_global.auth.current_user(request),
          flush=True)
    db_connection = await request.app.db_pool.acquire()
    media_data = await request.app.db_functions.db_media_new(days_old=7,
                                                             db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'data_new_media': media_data,
        'data_user_media_queue': False
    }

 */
