use crate::AppState;
use crate::axum_custom_filters::filters;
use crate::mk_lib_database;
use crate::user_preferences;
use askama::Template;
use axum::extract::Query;
use axum::extract::State;
use axum::response::Redirect;
use axum::response::Response;
use axum::{
    Extension,
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session::{SessionConfig, SessionLayer};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use core::fmt::Write;
use paginator::{PageItem, Paginator};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::{PgPool, PgRow};
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;
use sqlx::{FromRow, Row};

#[derive(Debug, Deserialize, Serialize)]
pub struct Genre {
    pub name: String,
    pub query_value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FilterOption {
    pub label: String,
    pub query_value: String,
}

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Debug, Deserialize, Serialize)]
struct TemplateMetaMovieList {
    template_metadata_guid: uuid::Uuid,
    template_metadata_name: String,
    template_metadata_name_alt: Option<String>,
    template_metadata_date: String,
    template_metadata_poster: String,
    template_metadata_runtime: i32,
    template_metadata_rating: String,
    template_metadata_star_rating: f32,
    template_metadata_availability: String,
    template_metadata_tagline: Option<String>,
    template_metadata_photo_updated: DateTime<Utc>,
    template_metadata_genre: Vec<Genre>,
    template_metadata_user_watched: serde_json::Value,
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
    page_title: Option<String>,
    pub current: Option<String>,
    pub genre_filter: Option<String>,
    pub genre_filter_query: Option<String>,
    pub primary_language_filter: Option<String>,
    pub status_filter: Option<String>,
    pub status_options: &'a Vec<FilterOption>,
    pub primary_language_options: &'a Vec<FilterOption>,
    pub filter_query_suffix: String,
    pub base_path: String,
}

#[derive(Debug, Deserialize)]
pub struct FilterQuery {
    pub starts_with: Option<String>,
    pub genre: Option<String>,
    pub primary_language: Option<String>,
    pub status: Option<String>,
}

fn normalize_starts_with(raw: Option<&str>) -> Option<String> {
    let value = raw.map(str::trim).filter(|value| !value.is_empty())?;
    let first_char = value.chars().next()?;

    if first_char == '#' {
        return Some("#".to_string());
    }

    if first_char.is_ascii_alphanumeric() {
        return Some(first_char.to_ascii_uppercase().to_string());
    }

    Some("#".to_string())
}

fn normalize_string_filter(raw: Option<&str>) -> Option<String> {
    raw.map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn normalize_status_filter(raw: Option<&str>) -> Option<String> {
    let value = raw.map(str::trim).filter(|value| !value.is_empty())?;
    match value {
        "favorite" | "watched" | "unwatched" | "good" | "bad" | "trash" => Some(value.to_string()),
        _ => None,
    }
}

fn build_filter_query_suffix(
    starts_with: Option<&str>,
    genre: Option<&str>,
    primary_language: Option<&str>,
    status_filter: Option<&str>,
) -> String {
    let mut query_params: Vec<String> = Vec::new();
    if let Some(sw) = starts_with.filter(|sw| !sw.is_empty()) {
        if sw == "#" {
            query_params.push("starts_with=%23".to_string());
        } else {
            query_params.push(format!("starts_with={sw}"));
        }
    }
    if let Some(genre_name) = genre.filter(|genre_name| !genre_name.is_empty()) {
        query_params.push(format!("genre={}", urlencoding::encode(genre_name)));
    }
    if let Some(language) = primary_language.filter(|language| !language.is_empty()) {
        query_params.push(format!(
            "primary_language={}",
            urlencoding::encode(language)
        ));
    }
    if let Some(status) = status_filter.filter(|status| !status.is_empty()) {
        query_params.push(format!("status={}", urlencoding::encode(status)));
    }
    if query_params.is_empty() {
        String::new()
    } else {
        format!("?{}", query_params.join("&"))
    }
}

fn build_movie_pagination(
    total_items: i64,
    page: i64,
    pagination_count: i64,
    starts_with: Option<&str>,
    genre: Option<&str>,
    primary_language: Option<&str>,
    status_filter: Option<&str>,
) -> Result<String, std::fmt::Error> {
    let total_pages = if total_items > 0 {
        (total_items + pagination_count - 1) / pagination_count
    } else {
        0
    };

    if total_pages <= 0 {
        return Ok(String::new());
    }

    let mut pagination_html = String::from(
        r#"<nav class="mt-6 flex justify-center" aria-label="Pagination">
<ul class="flex items-center gap-1 whitespace-nowrap text-sm">"#,
    );

    let suffix = build_filter_query_suffix(starts_with, genre, primary_language, status_filter);

    if total_pages == 1 {
        write!(
            pagination_html,
            r#"<li><a href="/user/metadata/movie/1{suffix}"
class="px-3 py-2 rounded-md bg-indigo-600 text-white font-semibold border border-indigo-600">1</a></li>"#,
        )?;
        pagination_html.push_str("</ul></nav>");
        return Ok(pagination_html);
    }

    let paginator = Paginator::builder(total_pages as usize)
        .current_page(page.max(1) as usize)
        .build_paginator()
        .map_err(|_| std::fmt::Error)?;

    for item in paginator.paginate() {
        match item {
            PageItem::Prev(p) => {
                write!(
                    pagination_html,
                    r#"<li><a href="/user/metadata/movie/{p}{suffix}"
class="px-3 py-2 rounded-md border border-gray-300 text-gray-700 hover:bg-gray-200"
aria-label="Previous">&laquo;</a></li>"#,
                )?;
            }
            PageItem::Page(p) => {
                write!(
                    pagination_html,
                    r#"<li><a href="/user/metadata/movie/{p}{suffix}"
class="px-3 py-2 rounded-md border border-gray-300 text-gray-700 hover:bg-gray-200">{p}</a></li>"#,
                )?;
            }
            PageItem::CurrentPage(p) => {
                write!(
                    pagination_html,
                    r#"<li><a href="/user/metadata/movie/{p}{suffix}"
class="px-3 py-2 rounded-md bg-indigo-600 text-white font-semibold border border-indigo-600"
aria-current="page">{p}</a></li>"#,
                )?;
            }
            PageItem::Ignore => {
                pagination_html
                    .push_str(r#"<li><span class="px-3 py-2 text-gray-400">…</span></li>"#);
            }
            PageItem::Next(p) => {
                write!(
                    pagination_html,
                    r#"<li><a href="/user/metadata/movie/{p}{suffix}"
class="px-3 py-2 rounded-md border border-gray-300 text-gray-700 hover:bg-gray-200"
aria-label="Next">&raquo;</a></li>"#,
                )?;
            }
            _ => {}
        }
    }

    pagination_html.push_str("</ul></nav>");
    Ok(pagination_html)
}

// pub async fn movies(
//     Query(params): Query<FilterQuery>,
// ) -> Html<String> {
//     let current = params.starts_with.clone();
//     // SQL filtering example
//     // "#" = symbols
//     // "A" = starts with A
//     // "5" = starts with number
//     Html(format!("Selected: {:?}", current))
// }

pub async fn user_metadata_movie(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(page): Path<i64>,
    Query(params): Query<FilterQuery>,
) -> impl IntoResponse {
    let starts_with = normalize_starts_with(params.starts_with.as_deref());
    let genre = normalize_string_filter(params.genre.as_deref());
    let primary_language = normalize_string_filter(params.primary_language.as_deref())
        .map(|value| value.to_lowercase());
    let status_filter = normalize_status_filter(params.status.as_deref());
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
        let current_user = auth.current_user.clone().unwrap_or_default();
        let pagination_count =
            user_preferences::load_user_pagination_count(&state.sqlx_pool_ro, current_user.id)
                .await
                .unwrap_or(user_preferences::DEFAULT_PAGINATION_COUNT);
        let db_offset: i64 = (page * pagination_count) - pagination_count;
        let total_pages: i64 =
        mk_lib_database::database_metadata::mk_lib_database_metadata_movie::mk_lib_database_metadata_movie_count(
           &state.sqlx_pool_ro,
            String::new(),
            current_user.id,
            starts_with.clone().unwrap_or_default(),
            genre.clone().unwrap_or_default(),
            primary_language.clone().unwrap_or_default(),
            status_filter.clone().unwrap_or_default(),
        )
        .await
        .unwrap();
        let pagination_html = build_movie_pagination(
            total_pages,
            page,
            pagination_count,
            starts_with.as_deref(),
            genre.as_deref(),
            primary_language.as_deref(),
            status_filter.as_deref(),
        )
        .unwrap();
        let primary_language_options: Vec<FilterOption> =
            mk_lib_database::mk_lib_database_language::mk_lib_database_language_read(
                &state.sqlx_pool_ro,
            )
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|row| FilterOption {
                label: row.language,
                query_value: row.code.to_lowercase(),
            })
            .collect();
        let status_options = vec![
            FilterOption {
                label: "Favorite".to_string(),
                query_value: "favorite".to_string(),
            },
            FilterOption {
                label: "Watched".to_string(),
                query_value: "watched".to_string(),
            },
            FilterOption {
                label: "Unwatched".to_string(),
                query_value: "unwatched".to_string(),
            },
            FilterOption {
                label: "Good".to_string(),
                query_value: "good".to_string(),
            },
            FilterOption {
                label: "Bad".to_string(),
                query_value: "bad".to_string(),
            },
            FilterOption {
                label: "Trash".to_string(),
                query_value: "trash".to_string(),
            },
        ];
        let filter_query_suffix = build_filter_query_suffix(
            starts_with.as_deref(),
            genre.as_deref(),
            primary_language.as_deref(),
            status_filter.as_deref(),
        );
        let movie_list =
        mk_lib_database::database_metadata::mk_lib_database_metadata_movie::mk_lib_database_metadata_movie_read(
            &state.sqlx_pool_ro,
            String::new(),
            current_user.id,
            starts_with.clone().unwrap_or_default(),
            genre.clone().unwrap_or_default(),
            primary_language.clone().unwrap_or_default(),
            status_filter.clone().unwrap_or_default(),
            db_offset,
            pagination_count,
        )
        .await
        .unwrap();
        let mut template_data_vec: Vec<TemplateMetaMovieList> = Vec::new();
        for row_data in movie_list.iter() {
            let watched_status = row_data.mm_status_user_json.clone().unwrap_or_else(|| {
                json!({
                    "bad": false,
                    "good": false,
                    "trash": false,
                    "watched": false,
                    "favorite": false
                })
            });
            let mut request_status: serde_json::Value = json!(false);
            let mut rating_status: serde_json::Value = json!(null);
            let mut queue_status: serde_json::Value = json!(false);

            // if !row_data.mm_metadata_user_json.is_none()
            //     && row_data
            //         .mm_metadata_user_json
            //         .as_ref()
            //         .unwrap()
            //         .get("UserStats")
            //         .is_some()
            // {
            //     let rating_json: serde_json::Value =
            //         row_data.mm_metadata_user_json.as_ref().unwrap().clone();
            //     rating_status =
            //         rating_json["UserStats"][current_user.id.to_string()]["Rating"].clone();
            //     watched_status =
            //         rating_json["UserStats"][current_user.id.to_string()]["Watched"].clone();
            //     request_status =
            //         rating_json["UserStats"][current_user.id.to_string()]["Request"].clone();
            //     queue_status =
            //         rating_json["UserStats"][current_user.id.to_string()]["Queue"].clone();
            // }

            let mut mm_poster: String = "/static/image/Movie-icon.png".to_string();
            if row_data.mm_poster.len() > 0 {
                mm_poster = row_data.mm_poster.clone();
            }

            let genres: Vec<Genre> = row_data
                .mm_genre
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|g| {
                    g.get("name").and_then(|n| n.as_str()).map(|name| Genre {
                        name: name.to_string(),
                        query_value: urlencoding::encode(name).into_owned(),
                    })
                })
                .collect();

            let temp_meta_line = TemplateMetaMovieList {
                template_metadata_guid: row_data.mm_metadata_movie_guid,
                template_metadata_name: row_data.mm_metadata_movie_name.clone(),
                template_metadata_name_alt: row_data.mm_metadata_movie_name_alt.clone(),
                template_metadata_date: row_data.mm_date.clone(),
                template_metadata_poster: mm_poster,
                template_metadata_runtime: row_data.mm_metadata_runtime,
                template_metadata_rating: "pg13".to_string(),
                template_metadata_star_rating: row_data.mm_metadata_vote_average.unwrap_or(0.0)
                    as f32,
                template_metadata_availability: row_data.mm_availibility.clone(),
                template_metadata_tagline: row_data.mm_metadata_tagline.clone(),
                template_metadata_photo_updated: row_data.photo_updated.clone(),
                template_metadata_genre: genres,
                template_metadata_user_watched: watched_status,
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
            page_title: Some("MediaKraken Metadata Movies".to_string()),
            current: starts_with.clone(),
            genre_filter: genre.clone(),
            genre_filter_query: genre
                .as_ref()
                .map(|value| urlencoding::encode(value).into_owned()),
            primary_language_filter: primary_language.clone(),
            status_filter: status_filter.clone(),
            status_options: &status_options,
            primary_language_options: &primary_language_options,
            filter_query_suffix,
            base_path: "/user/metadata/movie".to_string(),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_movie_detail.html")]
struct TemplateMetaMovieDetailContext<'a> {
    template_data_json: &'a serde_json::Value,
    template_metadata_name_alt: Option<String>,
    template_metadata_poster: String,
    template_metadata_backdrop: String,
    template_metadata_rating: String,
    template_metadata_star_rating: f32,
    template_metadata_availability: String,
    template_metadata_tagline: Option<String>,
    template_metadata_photo_updated: DateTime<Utc>,
    template_metadata_genre: Vec<Genre>,
    template_metadata_user_watched: serde_json::Value,
    template_metadata_user_rating: serde_json::Value,
    template_metadata_user_request: serde_json::Value,
    template_metadata_user_queue: serde_json::Value,
    page_title: Option<String>,
}

pub async fn user_metadata_movie_detail(
    State(state): State<AppState>,
    Path(guid): Path<uuid::Uuid>,
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
        let movie_metadata =
        mk_lib_database::database_metadata::mk_lib_database_metadata_movie::mk_lib_database_metadata_movie_detail_by_guid(
            &state.sqlx_pool_ro, guid, current_user.id
        )
        .await
        .unwrap();

        let watched_status = movie_metadata
            .mm_status_user_json
            .clone()
            .unwrap_or_else(|| {
                json!({
                    "bad": false,
                    "good": false,
                    "trash": false,
                    "watched": false,
                    "favorite": false
                })
            });
        let mut request_status: serde_json::Value = json!(false);
        let mut rating_status: serde_json::Value = json!(null);
        let mut queue_status: serde_json::Value = json!(false);

        let mut mm_poster: String = "/static/image/Movie-icon.png".to_string();
        if movie_metadata.mm_poster.len() > 0 {
            mm_poster = movie_metadata.mm_poster.clone();
        }

        let genres: Vec<Genre> = movie_metadata
            .mm_genre
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|g| {
                g.get("name").and_then(|n| n.as_str()).map(|name| Genre {
                    name: name.to_string(),
                    query_value: urlencoding::encode(name).into_owned(),
                })
            })
            .collect();

        let template = TemplateMetaMovieDetailContext {
            template_data_json: &movie_metadata.mm_metadata_movie_json,
            template_metadata_name_alt: movie_metadata.mm_metadata_movie_name_alt.clone(),
            template_metadata_poster: mm_poster,
            template_metadata_backdrop: "/static/image/Movie-icon.png".to_string(),
            template_metadata_rating: "pg13".to_string(),
            template_metadata_star_rating: movie_metadata.mm_metadata_vote_average.unwrap_or(0.0)
                as f32,
            template_metadata_availability: movie_metadata.mm_availibility.clone(),
            template_metadata_tagline: movie_metadata.mm_metadata_tagline.clone(),
            template_metadata_photo_updated: movie_metadata.photo_updated.clone(),
            template_metadata_genre: genres,
            template_metadata_user_watched: watched_status,
            template_metadata_user_rating: rating_status,
            template_metadata_user_request: request_status,
            template_metadata_user_queue: queue_status,
            page_title: Some("MediaKraken Metadata Movie Detail".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

pub async fn user_metadata_movie_status(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    axum::Json(payload): axum::Json<mk_lib_database::mk_lib_database::MediaStatusUpdatePayload>,
) -> impl IntoResponse {
    let current_user = auth.current_user.clone().unwrap_or_default();
    if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
        [Method::POST],
        false,
    )
    .requires(Rights::any([Rights::permission("User::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        println!("here is the user: {:?}", current_user.id);
        // let template = TemplateError401Context {};
        // let reply_html = template.render().unwrap();
        return StatusCode::UNAUTHORIZED.into_response();
    } else {
        println!("here is the payload4: {:?}", payload);
        let _row_data = mk_lib_database::database_metadata::mk_lib_database_metadata_movie::mk_lib_database_metadata_movie_status(
            &state.sqlx_pool_rw, payload, current_user.id
        )
        .await
        .unwrap();
        StatusCode::OK.into_response()
    }
}

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
 */
