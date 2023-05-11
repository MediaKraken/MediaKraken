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
#[template(path = "bss_user/playback/bss_user_playback_comic.html")]
struct UserPlaybackComicTemplate;

pub async fn user_playback_comic(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let template = UserPlaybackComicTemplate {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

/*
@blueprint_user_playback_comic.route('/user_comic_view/<guid>', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/bss_user_playback_comic.html')
@common_global.auth.login_required
pub async fn url_bp_user_playback_comic(request, guid):
    """
    Display image comic view
    """
    db_connection = await request.app.db_pool.acquire()
    comic_data = await request.app.db_functions.db_media_path_by_uuid(guid, db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        comic_data: comic_data,
    }

 */
