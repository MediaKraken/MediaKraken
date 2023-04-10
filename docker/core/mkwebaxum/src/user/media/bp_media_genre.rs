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

mod filters {
    pub fn space_to_html(s: &str) -> ::askama::Result<String> {
        Ok(s.replace(" ", "%20"))
    }
}

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_genre_video.html")]
struct TemplateUserGenreContext {}

pub async fn user_media_genre(
    Extension(sqlx_pool): Extension<PgPool>,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
) -> impl IntoResponse {
    let template = TemplateUserGenreContext {};
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

/*

@blueprint_user_media_genre.route("/user_media_genre", methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_genre_video.html')
@common_global.auth.login_required
pub async fn url_bp_user_media_genre(request):
    """
    Display media split up by genre
    """
    media = []
    db_connection = await request.app.db_pool.acquire()
    for row_data in await request.app.db_functions.db_media_movie_count_by_genre(
            common_global.DLMediaType.Movie.value, db_connection=db_connection):
        print('genre:', row_data, flush=True)
        media.append((row_data['gen']['name'],
                      common_internationalization.com_inter_number_format(row_data['gen_count']),
                      row_data['gen']['name'] + ".png"))
    await request.app.db_pool.release(db_connection)
    return {
        'media': sorted(media)
    }


@blueprint_user_media_genre.route("/user_movie/<genre>", methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_movie.html')
@common_global.auth.login_required(user_keyword='user')
pub async fn url_bp_user_movie_page(request, user, genre):
    """
    Display movie page
    """
    print('current user - url_bp_user_movie_page', common_global.auth.current_user(request),
          flush=True)
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    media = []
    db_connection = await request.app.db_pool.acquire()
    for row_data in await request.app.db_functions.db_media_movie_list(
            common_global.DLMediaType.Movie.value,
            list_type='movie',
            list_genre=genre,
            list_limit=int(request.ctx.session['per_page']),
            group_collection=False,
            offset=offset,
            include_remote=True,
            search_text=request.ctx.session['search_text'],
            db_connection=db_connection):
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text=
                                                                         {"row2": row_data[
                                                                             'mm_metadata_user_json']})
        # set watched
        try:
            watched_status \
                = row_data['mm_metadata_user_json']['UserStats'][user.id]['watched']
        except (KeyError, TypeError):
            watched_status = False
        # set synced
        try:
            sync_status = \
                row_data['mm_metadata_user_json']['UserStats'][user.id]['sync']
        except (KeyError, TypeError):
            sync_status = False
        # set rating
        if row_data['mm_metadata_user_json'] != None \
                and 'UserStats' in row_data['mm_metadata_user_json'] \
                and user.id in row_data['mm_metadata_user_json']['UserStats'] \
                and 'Rating' in row_data['mm_metadata_user_json']['UserStats'][user.id]:
            rating_status \
                = row_data['mm_metadata_user_json']['UserStats'][user.id]['Rating']
            if rating_status == 'favorite':
                rating_status = 'favorite-mark.png'
            else if rating_status == 'like':
                rating_status = 'thumbs-up.png'
            else if rating_status == 'dislike':
                rating_status = 'dislike-thumb.png'
            else if rating_status == 'poo':
                rating_status = 'pile-of-dung.png'
        else:
            rating_status = None
        # set mismatch
        try:
            match_status = row_data['mismatch']
        except (KeyError, TypeError):
            match_status = False
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             "status": watched_status,
                                                                             'sync': sync_status,
                                                                             'rating': rating_status,
                                                                             'match': match_status})
        media.append((row_data['mm_media_name'], row_data['mm_media_guid'],
                      row_data['mm_poster'],
                      watched_status, sync_status, rating_status, match_status))
    total = await request.app.db_functions.db_media_movie_list_count(
        common_global.DLMediaType.Movie.value,
        list_type='movie',
        list_genre=genre,
        group_collection=False,
        include_remote=True,
        search_text=
        request.ctx.session['search_text'],
        db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    request.ctx.session['search_page'] = 'media_movie'
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_movie',
                                                                      item_count=total,
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    return {
        'media': media,
        'pagination_bar': pagination,
    }

 */
