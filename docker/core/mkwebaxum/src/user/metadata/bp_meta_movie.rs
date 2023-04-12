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
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgPool;
use stdext::function_name;

mod filters {
    pub fn space_to_html(s: &str) -> ::askama::Result<String> {
        Ok(s.replace(" ", "%20"))
    }
}

use crate::mk_lib_logging;

#[path = "../../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../../mk_lib_database_metadata_movie.rs"]
mod mk_lib_database_metadata_movie;

use crate::mk_lib_database_user;

#[derive(Debug, Deserialize, Serialize)]
struct TemplateMetaMovieList {
    template_metadata_guid: uuid::Uuid,
    template_metadata_name: String,
    template_metadata_date: String,
    template_metadata_poster: String,
    template_metadata_user_watched: serde_json::Value,
    template_metadata_user_rating_view: bool,
    template_metadata_user_rating: serde_json::Value,
    template_metadata_user_request: serde_json::Value,
    template_metadata_user_queue: serde_json::Value,
}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_movie.html")]
struct TemplateMetaMovieContext<'a> {
    template_data: &'a Vec<TemplateMetaMovieList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
}

pub async fn user_metadata_movie(
    Extension(sqlx_pool): Extension<PgPool>,
    auth: AuthSession<mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(page): Path<i64>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    let db_offset: i64 = (page * 30) - 30;
    let total_pages: i64 = mk_lib_database_metadata_movie::mk_lib_database_metadata_movie_count(
        &sqlx_pool,
        String::new(),
    )
    .await
    .unwrap();
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
        total_pages,
        page,
        "/user/metadata/movie".to_string(),
    )
    .await
    .unwrap();
    let movie_list = mk_lib_database_metadata_movie::mk_lib_database_metadata_movie_read(
        &sqlx_pool,
        String::new(),
        db_offset,
        30,
    )
    .await
    .unwrap();
    let mut template_data_vec: Vec<TemplateMetaMovieList> = Vec::new();
    for row_data in movie_list.iter() {
        let mut watched_status: serde_json::Value = json!(false);
        let mut request_status: serde_json::Value = json!(false);
        let mut rating_status: serde_json::Value = json!(null);
        let mut queue_status: serde_json::Value = json!(false);
        if !row_data.mm_metadata_user_json.is_none()
            && row_data
                .mm_metadata_user_json
                .as_ref()
                .unwrap()
                .get("UserStats")
                .is_some()
        {
            let rating_json: serde_json::Value =
                row_data.mm_metadata_user_json.as_ref().unwrap().clone();
            rating_status = rating_json["UserStats"][current_user.id.to_string()]["Rating"].clone();
            watched_status =
                rating_json["UserStats"][current_user.id.to_string()]["Watched"].clone();
            request_status =
                rating_json["UserStats"][current_user.id.to_string()]["Request"].clone();
            queue_status = rating_json["UserStats"][current_user.id.to_string()]["Queue"].clone();
        }
        let mut mm_poster: String = "/image/Movie-icon.png".to_string();
        if row_data.mm_poster.len() > 0 {
            mm_poster = row_data.mm_poster.clone();
        }
        let temp_meta_line = TemplateMetaMovieList {
            template_metadata_guid: row_data.mm_metadata_guid,
            template_metadata_name: row_data.mm_metadata_name.clone(),
            template_metadata_date: row_data.mm_date.clone(),
            template_metadata_poster: mm_poster,
            template_metadata_user_watched: watched_status,
            template_metadata_user_rating_view: false,
            template_metadata_user_rating: rating_status,
            template_metadata_user_request: request_status,
            template_metadata_user_queue: queue_status,
        };
        template_data_vec.push(temp_meta_line);
    }
    let mut template_data_exists = false;
    if template_data_vec.len() > 0 {
        template_data_exists = true;
    }
    let page_usize = page as usize;
    let template = TemplateMetaMovieContext {
        template_data: &template_data_vec,
        template_data_exists: &template_data_exists,
        pagination_bar: &pagination_html,
        page: &page_usize,
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

// #[derive(Template)]
// #[template(path = "bss_user/metadata/bss_user_metadata_movie_detail.html")]
// struct TemplateMetaMovieDetailContext {
//     template_data_json: serde_json::Value,
//     template_data_json_media_ffmpeg: serde_json::Value,
//     template_data_json_media_crew: serde_json::Value,
// }

// pub async fn user_metadata_movie_detail(
//     Extension(sqlx_pool): Extension<PgPool>,
//     Path(guid): Path<uuid::Uuid>,
// ) -> impl IntoResponse {
//     let movie_metadata =
//         mk_lib_database_metadata_movie::mk_lib_database_metadata_movie_detail_by_guid(
//             &sqlx_pool, guid,
//         )
//         .await
//         .unwrap();
//     let template = TemplateMetaMovieDetailContext {
//         template_data_json: movie_metadata.mm_metadata_movie_json,
//         template_data_json_media_ffmpeg: json!({ None }),
//         template_data_json_media_crew: json!({ None }),
//     };
//     let reply_html = template.render().unwrap();
//     (StatusCode::OK, Html(reply_html).into_response())
// }

/*

@blueprint_user_metadata_movie.route('/user_meta_movie_detail/<guid>')
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_movie_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_movie_detail(request, guid):
    """
    Display metadata movie detail
    """
    db_connection = await request.app.db_pool.acquire()
    data = await request.app.db_functions.db_meta_movie_detail(media_guid=guid,
                                                               db_connection=db_connection)
    # vote count format
    try:
        data_vote_count = common_internationalization.com_inter_number_format(
            data['mm_metadata_json']['vote_count'])
    except:
        data_vote_count = 'NA'
    # build gen list
    genres_list = ''
    for ndx in range(0, len(data['mm_metadata_json']['genres'])):
        genres_list += (data['mm_metadata_json']['genres'][ndx]['name'] + ', ')
    # build production list
    production_list = ''
    for ndx in range(0, len(data['mm_metadata_json']['production_companies'])):
        production_list \
            += (data['mm_metadata_json']['production_companies'][ndx]['name'] + ', ')
    # poster image
    try:
        if data['mm_metadata_localimage_json']['Poster'] != None:
            data_poster_image = data['mm_metadata_localimage_json']['Poster']
        else:
            data_poster_image = None
    except:
        data_poster_image = None
    # background image
    try:
        if data['mm_metadata_localimage_json']['Backdrop'] != None:
            data_background_image = data['mm_metadata_localimage_json']['Backdrop']
        else:
            data_background_image = None
    except:
        data_background_image = None
    # grab reviews
    review = await request.app.db_functions.db_review_list_by_meta_guid(metadata_id=guid,
                                                                        db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'data_name': data['mm_metadata_name'],
        'json_metadata': data['mm_metadata_json'],
        'data_genres': genres_list[:-2],
        'data_production': production_list[:-2],
        'data_review': review,
        'data_poster_image': data_poster_image,
        'data_background_image': data_background_image,
        'data_vote_count': data_vote_count,
        'data_budget': common_internationalization.com_inter_number_format(
            data['mm_metadata_json']['budget'])
    }


@blueprint_user_metadata_movie.route('/user_meta_movie_list', methods=["GET", "POST"])
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_movie.html')
@common_global.auth.login_required(user_keyword='user')
pub async fn url_bp_user_metadata_movie_list(request, user):
    """
    Display list of movie metadata
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    media = []
    media_count = 0
    db_connection = await request.app.db_pool.acquire()
    for row_data in await request.app.db_functions.db_meta_movie_list(offset,
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      request.ctx.session[
                                                                          'search_text'],
                                                                      db_connection):
        if row_data['mm_metadata_user_json'] != None:
            user_json = row_data['mm_metadata_user_json']
        else:
            user_json = None
        # set watched
        try:
            watched_status = user_json['UserStats'][str(user.id)]['watched']
        except (KeyError, TypeError):
            watched_status = False
        # set rating
        if user_json != None \
                and 'UserStats' in user_json \
                and str(user.id) in user_json['UserStats'] \
                and 'Rating' in user_json['UserStats'][str(user.id)]:
            rating_status \
                = user_json['UserStats'][str(user.id)]['Rating']
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
        # set requested
        try:
            request_status = user_json['UserStats'][str(user.id)]['requested']
        except (KeyError, TypeError):
            request_status = None
        # set queue
        try:
            queue_status = user_json['UserStats'][str(user.id)]['queue']
        except (KeyError, TypeError):
            queue_status = None
        await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                         message_text={
                                                                             "status": watched_status,
                                                                             'rating': rating_status,
                                                                             'request': request_status,
                                                                             'queue': queue_status})
        media_count += 1
        if media_count == 1:
            deck_start = True
        else:
            deck_start = False
        if media_count == 4:
            deck_break = True
            media_count = 0
        else:
            deck_break = False
        media.append((row_data['mm_metadata_guid'], row_data['mm_metadata_name'],
                      row_data['mm_date'], row_data['mm_poster'], watched_status,
                      rating_status, request_status, queue_status, deck_start, deck_break))
    request.ctx.session['search_page'] = 'meta_movie'
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page=page,
                                                                      url='/user/user_meta_movie_list',
                                                                      item_count=await request.app.db_functions.db_meta_movie_count(
                                                                          request.ctx.session[
                                                                              'search_text'],
                                                                          db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    await request.app.db_pool.release(db_connection)
    return {
        'media_movie': media,
        'pagination_bar': pagination,
    }


@blueprint_user_metadata_movie.route('/user_meta_movie_status/<guid>/<event_type>',
                                     methods=['GET', 'POST'])
@common_global.auth.login_required(user_keyword='user')
pub async fn url_bp_user_metadata_movie_status(request, user, guid, event_type):
    """
    Set media status for specified media, user
    """
    await common_logging_elasticsearch_httpx.com_es_httpx_post_async(message_type='info',
                                                                     message_text={
                                                                         'movie metadata status': guid,
                                                                         'event': event_type})
    db_connection = await request.app.db_pool.acquire()
    await request.app.db_functions.db_meta_movie_status_update(guid,
                                                               user.id, event_type,
                                                               db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return response.HTTPResponse('', status=200, headers={'Vary': 'Accept-Encoding'})

 */
