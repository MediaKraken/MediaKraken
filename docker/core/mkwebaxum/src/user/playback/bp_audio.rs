use askama::Template;
use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Router,
};
use axum_session_auth::{Auth, AuthSession, Rights, SessionPgPool};
use mk_lib_database;
use serde_json::json;
use sqlx::postgres::PgPool;
use stdext::function_name;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_user/playback/bss_user_playback_album.html")]
struct UserPlaybackAlbumTemplate;

pub async fn user_playback_audio(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::GET],
        false,
    )
    .requires(Rights::any([Rights::permission("User::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        let template = TemplateError401Context {};
        let reply_html = template.render().unwrap();
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let template = UserPlaybackAlbumTemplate {};
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

/*
@blueprint_user_playback_audio.route('/user_play_album/<guid>', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/bss_user_album_playback.html')
@common_global.auth.login_required
pub async fn url_bp_user_playback_album(request, guid):
    """
    Obsolete?
    """
    db_connection = await request.app.db_pool.acquire()
    data_desc = await request.app.db_functions.db_meta_music_album_by_guid(guid, db_connection=db_connection)
    data_song_list = await request.app.db_functions.db_meta_music_songs_by_album_guid(guid,
                                                                                      db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        data_desc: data_desc,
        data_song_list: data_song_list,
    }

 */
