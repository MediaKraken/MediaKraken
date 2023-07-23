use askama::Template;
use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use axum_session_auth::{AuthSession, SessionPgPool};
use mk_lib_database;
use serde_json::json;
use sqlx::postgres::PgPool;
use stdext::function_name;

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_image_gallery.html")]
struct TemplateUserImageContext {}

pub async fn user_media_image(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let template = TemplateUserImageContext {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

/*
@blueprint_user_image.route('/user_imagegallery')
@common_global.jinja_template.template('bss_user/media/bss_user_media_image_gallery.html')
@common_global.auth.login_required
pub async fn url_bp_user_image_gallery(request):
    """
    Display image gallery page
    """
    db_connection = await request.app.db_pool.acquire()
    image_data = await request.app.db_functions.db_image_list(common_global.DLMediaType.Picture,
                                                              db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {'image_data': image_data
            }

 */
