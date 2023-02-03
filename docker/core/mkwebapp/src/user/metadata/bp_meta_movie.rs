#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use rocket::response::Redirect;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::Request;
use rocket_auth::{Auth, Error, Login, Signup, User, Users};
use rocket_dyn_templates::{tera::Tera, Template};
use serde_json::json;
use stdext::function_name;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../../mk_lib_common_pagination.rs"]
mod mk_lib_common_pagination;

#[path = "../../mk_lib_database_metadata_movie.rs"]
mod mk_lib_database_metadata_movie;

#[derive(Serialize)]
struct TemplateMetaMovieContext {
    template_data: Vec<TemplateMetaMovieList>,
    pagination_bar: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TemplateMetaMovieList {
    template_metadata_guid: uuid::Uuid,
    template_metadata_name: String,
    template_metadata_date: String,
    template_metadata_poster: Option<String>,
    template_metadata_user_watched: serde_json::Value,
    template_metadata_user_rating: Option<serde_json::Value>,
    template_metadata_user_request: serde_json::Value,
    template_metadata_user_queue: serde_json::Value,
}

#[get("/metadata/movie/<page>")]
pub async fn user_metadata_movie(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    user: User,
    page: i32,
) -> Template {
    let db_offset: i32 = (page * 30) - 30;
    let mut total_pages: i64 =
        mk_lib_database_metadata_movie::mk_lib_database_metadata_movie_count(
            &sqlx_pool,
            String::new(),
        )
        .await
        .unwrap();
    if total_pages > 0 {
        total_pages = total_pages / 30;
    }
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
        if !row_data.mm_metadata_user_json.is_none() && row_data
            .mm_metadata_user_json
            .as_ref()
            .unwrap()
            .get("UserStats")
            .is_some()
        {
            let rating_json: serde_json::Value =
                row_data.mm_metadata_user_json.as_ref().unwrap().clone();
            rating_status = rating_json["UserStats"][user.id().to_string()]["Rating"].clone();
            watched_status = rating_json["UserStats"][user.id().to_string()]["Watched"].clone();
            request_status = rating_json["UserStats"][user.id().to_string()]["Request"].clone();
            queue_status = rating_json["UserStats"][user.id().to_string()]["Queue"].clone();
        }
        let temp_meta_line = TemplateMetaMovieList {
            template_metadata_guid: row_data.mm_metadata_guid,
            template_metadata_name: row_data.mm_metadata_name.clone(),
            template_metadata_date: row_data.mm_date.clone(),
            template_metadata_poster: Some(row_data.mm_poster.clone()),
            template_metadata_user_watched: watched_status,
            template_metadata_user_rating: Some(rating_status),
            template_metadata_user_request: request_status,
            template_metadata_user_queue: queue_status,
        };
        template_data_vec.push(temp_meta_line);
    }
    Template::render(
        "bss_user/metadata/bss_user_metadata_movie",
        &TemplateMetaMovieContext {
            template_data: template_data_vec,
            pagination_bar: pagination_html,
        },
    )
}

#[derive(Serialize)]
struct TemplateMetaMovieDetailContext {
    template_data: serde_json::Value,
}

#[get("/metadata/movie_detail/<guid>")]
pub async fn user_metadata_movie_detail(
    sqlx_pool: &rocket::State<sqlx::PgPool>,
    user: User,
    guid: rocket::serde::uuid::Uuid,
) -> Template {
    let tmp_uuid = sqlx::types::Uuid::parse_str(&guid.to_string()).unwrap();
    Template::render(
        "bss_user/metadata/bss_user_metadata_movie_detail",
        tera::Context::new().into_json(),
    )
}

/*
from common import common_global
from common import common_internationalization
from common import common_logging_elasticsearch_httpx
from common import common_pagination_bootstrap
from sanic import Blueprint, response

blueprint_user_metadata_movie = Blueprint('name_blueprint_user_metadata_movie',
                                          url_prefix='/user')


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
