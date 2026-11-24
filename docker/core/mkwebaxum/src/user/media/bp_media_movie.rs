use crate::AppState;
use crate::mk_lib_database;
use crate::user_metadata::bp_meta_movie::{
    FilterOption, FilterQuery, Genre, MovieCastMember, MovieCrewMember, build_filter_query_suffix,
    build_movie_pagination, movie_credits, normalize_starts_with, normalize_status_filter,
    normalize_string_filter,
};
use crate::user_preferences;
use askama::Template;
use axum::extract::Query;
use axum::extract::State;
use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use mk_lib_common::mk_lib_common_enum_media_type::DLMediaType;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgPool;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

/// One movie with local media (a representative media file per title),
/// joined with the movie metadata so the media movie list can display the
/// same data as the metadata movie list.
#[derive(Debug, sqlx::FromRow)]
struct MediaMovieListRow {
    mm_media_guid: uuid::Uuid,
    mm_metadata_movie_guid: uuid::Uuid,
    mm_metadata_movie_name: String,
    mm_date: String,
    mm_poster: String,
    mm_status_user_json: Option<serde_json::Value>,
    mm_genre: serde_json::Value,
    mm_metadata_tagline: Option<String>,
    mm_metadata_runtime: i32,
    mm_metadata_vote_average: Option<f64>,
    photo_updated: DateTime<Utc>,
}

/// One card on the media movie list page.
#[derive(Debug, Deserialize, Serialize)]
struct TemplateMediaMovieList {
    template_media_guid: uuid::Uuid,
    template_metadata_guid: uuid::Uuid,
    template_metadata_name: String,
    template_metadata_date: String,
    template_metadata_poster: String,
    template_metadata_runtime: i32,
    template_metadata_rating: String,
    template_metadata_star_rating: f32,
    template_metadata_tagline: Option<String>,
    template_metadata_photo_updated: DateTime<Utc>,
    template_metadata_genre: Vec<Genre>,
    template_metadata_user_watched: serde_json::Value,
}

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_movie.html")]
struct TemplateMediaMovieContext<'a> {
    template_data: &'a Vec<TemplateMediaMovieList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    #[allow(dead_code)]
    page: &'a usize,
    page_title: Option<String>,
    pub current: Option<String>,
    pub genre_filter: Option<String>,
    pub genre_filter_query: Option<String>,
    pub primary_language_filter: Option<String>,
    pub status_filter: Option<String>,
    pub status_options: &'a Vec<FilterOption>,
    pub primary_language_options: &'a Vec<FilterOption>,
    #[allow(dead_code)]
    pub filter_query_suffix: String,
    pub base_path: String,
}

/// Count movies that have local media using the metadata movie list
/// filters. `starts_with` is "", "#", or a single character prefix; empty
/// genre, language, and status values disable the respective filter.
async fn media_movie_filter_count(
    sqlx_pool: &PgPool,
    user_id: i64,
    starts_with: &str,
    genre_name: &str,
    primary_language: &str,
    status_filter: &str,
) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        r#"select count(*)
            from (select distinct on (mm_metadata_movie_guid) mm_media_guid,
            mm_metadata_movie_guid
            from mm_media, mm_metadata_movie
            where mm_media_class_enum = $1
            and mm_media_metadata_guid = mm_metadata_movie_guid
            order by mm_metadata_movie_guid, mm_media_path) as temp
            join mm_metadata_movie
            on mm_metadata_movie.mm_metadata_movie_guid = temp.mm_metadata_movie_guid
            left join mm_metadata_user_status
            on mm_metadata_user_status.mm_status_type_movie = temp.mm_metadata_movie_guid
            and mm_metadata_user_status.mm_status_user_id = $2
            where (
                ($3 = '#' AND left(lower(mm_metadata_movie_name), 1) !~ '^[a-z0-9]$')
                OR ($3 <> '#' AND lower(mm_metadata_movie_name) LIKE lower($3) || '%')
            )
            AND (
                $4 = ''
                OR EXISTS (
                    SELECT 1
                    FROM jsonb_array_elements(mm_metadata_movie_json->'genres') AS genre
                    WHERE lower(genre->>'name') = lower($4)
                )
            )
            AND (
                $5 = ''
                OR lower(mm_metadata_movie_primary_lang_gen) = lower($5)
            )
            AND (
                $6 = ''
                OR (
                    $6 = 'unwatched'
                    AND NOT COALESCE((mm_status_user_json->>'watched')::boolean, false)
                )
                OR (
                    $6 <> 'unwatched'
                    AND COALESCE((mm_status_user_json->>$6)::boolean, false)
                )
            )"#,
    )
    .bind(DLMediaType::MOVIE)
    .bind(user_id)
    .bind(starts_with)
    .bind(genre_name)
    .bind(primary_language)
    .bind(status_filter)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

/// Read movies that have local media (one representative media file per
/// title) using the metadata movie list filters.
#[allow(clippy::too_many_arguments)]
async fn media_movie_filter_read(
    sqlx_pool: &PgPool,
    user_id: i64,
    starts_with: &str,
    genre_name: &str,
    primary_language: &str,
    status_filter: &str,
    offset: i64,
    limit: i64,
) -> Result<Vec<MediaMovieListRow>, sqlx::Error> {
    sqlx::query_as(
        r#"select temp.mm_media_guid,
            temp.mm_metadata_movie_guid,
            mm_metadata_movie_name,
            mm_metadata_movie_json->>'release_date' as mm_date,
            coalesce(mm_metadata_movie_localimage_json->>'Poster', '') as mm_poster,
            (mm_metadata_movie_json->'runtime')::int as mm_metadata_runtime,
            mm_metadata_movie_json->>'tagline' as mm_metadata_tagline,
            (mm_metadata_movie_json->'genres')::jsonb as mm_genre,
            mm_status_user_json,
            ROUND((mm_metadata_movie_json->'vote_average')::numeric, 1)::float as mm_metadata_vote_average,
            mm_metadata_movie.photo_updated
            from (select distinct on (mm_metadata_movie_guid) mm_media_guid,
            mm_metadata_movie_guid
            from mm_media, mm_metadata_movie
            where mm_media_class_enum = $1
            and mm_media_metadata_guid = mm_metadata_movie_guid
            order by mm_metadata_movie_guid, mm_media_path) as temp
            join mm_metadata_movie
            on mm_metadata_movie.mm_metadata_movie_guid = temp.mm_metadata_movie_guid
            left join mm_metadata_user_status
            on mm_metadata_user_status.mm_status_type_movie = temp.mm_metadata_movie_guid
            and mm_metadata_user_status.mm_status_user_id = $2
            where (
                ($3 = '#' AND left(lower(mm_metadata_movie_name), 1) !~ '^[a-z0-9]$')
                OR ($3 <> '#' AND lower(mm_metadata_movie_name) LIKE lower($3) || '%')
            )
            AND (
                $4 = ''
                OR EXISTS (
                    SELECT 1
                    FROM jsonb_array_elements(mm_metadata_movie_json->'genres') AS genre
                    WHERE lower(genre->>'name') = lower($4)
                )
            )
            AND (
                $5 = ''
                OR lower(mm_metadata_movie_primary_lang_gen) = lower($5)
            )
            AND (
                $6 = ''
                OR (
                    $6 = 'unwatched'
                    AND NOT COALESCE((mm_status_user_json->>'watched')::boolean, false)
                )
                OR (
                    $6 <> 'unwatched'
                    AND COALESCE((mm_status_user_json->>$6)::boolean, false)
                )
            )
            order by LOWER(mm_metadata_movie_name), mm_date
            offset $7 limit $8"#,
    )
    .bind(DLMediaType::MOVIE)
    .bind(user_id)
    .bind(starts_with)
    .bind(genre_name)
    .bind(primary_language)
    .bind(status_filter)
    .bind(offset)
    .bind(limit)
    .fetch_all(sqlx_pool)
    .await
}

pub async fn user_media_movie(
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
        let pagination_count =
            user_preferences::load_user_pagination_count(&state.sqlx_pool_ro, current_user.id)
                .await
                .unwrap_or(user_preferences::DEFAULT_PAGINATION_COUNT);
        let db_offset: i64 = (page * pagination_count) - pagination_count;
        let total_items: i64 = match media_movie_filter_count(
            &state.sqlx_pool_ro,
            current_user.id,
            starts_with.as_deref().unwrap_or_default(),
            genre.as_deref().unwrap_or_default(),
            primary_language.as_deref().unwrap_or_default(),
            status_filter.as_deref().unwrap_or_default(),
        )
        .await
        {
            Ok(total) => total,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(String::from("Internal server error")).into_response(),
                );
            }
        };
        let pagination_html = match build_movie_pagination(
            "/user/media/movie",
            total_items,
            page,
            pagination_count,
            starts_with.as_deref(),
            genre.as_deref(),
            primary_language.as_deref(),
            status_filter.as_deref(),
        ) {
            Ok(html) => html,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(String::from("Internal server error")).into_response(),
                );
            }
        };
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
        let movie_list = match media_movie_filter_read(
            &state.sqlx_pool_ro,
            current_user.id,
            starts_with.as_deref().unwrap_or_default(),
            genre.as_deref().unwrap_or_default(),
            primary_language.as_deref().unwrap_or_default(),
            status_filter.as_deref().unwrap_or_default(),
            db_offset,
            pagination_count,
        )
        .await
        {
            Ok(list) => list,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(String::from("Internal server error")).into_response(),
                );
            }
        };
        let mut template_data_vec: Vec<TemplateMediaMovieList> = Vec::new();
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
            let mut mm_poster: String = "/static/image/Movie-icon.png".to_string();
            if !row_data.mm_poster.is_empty() {
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
            template_data_vec.push(TemplateMediaMovieList {
                template_media_guid: row_data.mm_media_guid,
                template_metadata_guid: row_data.mm_metadata_movie_guid,
                template_metadata_name: row_data.mm_metadata_movie_name.clone(),
                template_metadata_date: row_data.mm_date.clone(),
                template_metadata_poster: mm_poster,
                template_metadata_runtime: row_data.mm_metadata_runtime,
                template_metadata_rating: "pg13".to_string(),
                template_metadata_star_rating: row_data.mm_metadata_vote_average.unwrap_or(0.0)
                    as f32,
                template_metadata_tagline: row_data.mm_metadata_tagline.clone(),
                template_metadata_photo_updated: row_data.photo_updated,
                template_metadata_genre: genres,
                template_metadata_user_watched: watched_status,
            });
        }
        let mut template_data_exists = false;
        if !template_data_vec.is_empty() {
            template_data_exists = true;
        }
        let page_usize = page as usize;
        let template = TemplateMediaMovieContext {
            template_data: &template_data_vec,
            template_data_exists: &template_data_exists,
            pagination_bar: &pagination_html,
            #[allow(dead_code)]
            page: &page_usize,
            page_title: Some("MediaKraken Movies".to_string()),
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
            base_path: "/user/media/movie".to_string(),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

#[derive(Debug, sqlx::FromRow)]
struct MediaMovieFileRow {
    mm_media_guid: uuid::Uuid,
    mm_media_path: String,
    mm_media_ffprobe_json: Option<serde_json::Value>,
}

/// One media file (version) of a movie with ffprobe-derived display values.
#[derive(Debug)]
struct TemplateMediaFileList {
    mm_media_guid: uuid::Uuid,
    file_name: String,
    resolution: String,
    runtime: String,
}

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_movie_detail.html")]
struct TemplateMediaMovieDetailContext<'a> {
    template_data_json: &'a serde_json::Value,
    template_metadata_guid: uuid::Uuid,
    #[allow(dead_code)]
    template_metadata_name_alt: Option<String>,
    template_metadata_poster: String,
    #[allow(dead_code)]
    template_metadata_backdrop: String,
    #[allow(dead_code)]
    template_metadata_rating: String,
    #[allow(dead_code)]
    template_metadata_star_rating: f32,
    #[allow(dead_code)]
    template_metadata_availability: String,
    template_metadata_tagline: Option<String>,
    template_metadata_photo_updated: DateTime<Utc>,
    template_metadata_genre: Vec<Genre>,
    template_metadata_cast: Vec<MovieCastMember>,
    template_metadata_crew: Vec<MovieCrewMember>,
    template_metadata_user_watched: serde_json::Value,
    #[allow(dead_code)]
    template_metadata_user_rating: serde_json::Value,
    #[allow(dead_code)]
    template_metadata_user_request: serde_json::Value,
    #[allow(dead_code)]
    template_metadata_user_queue: serde_json::Value,
    template_media_files: Vec<TemplateMediaFileList>,
    page_title: Option<String>,
}

/// Width x height of the first video stream in an ffprobe document, or "NA".
fn parse_ffprobe_resolution(ffprobe: &serde_json::Value) -> String {
    let Some(streams) = ffprobe.get("streams").and_then(serde_json::Value::as_array) else {
        return "NA".to_string();
    };
    for stream in streams {
        if stream.get("codec_type").and_then(serde_json::Value::as_str) == Some("video")
            && let (Some(width), Some(height)) = (
                stream.get("width").and_then(serde_json::Value::as_u64),
                stream.get("height").and_then(serde_json::Value::as_u64),
            )
        {
            return format!("{width}x{height}");
        }
    }
    "NA".to_string()
}

/// format.duration (seconds) formatted as HH:MM:SS, or "NA".
fn parse_ffprobe_runtime(ffprobe: &serde_json::Value) -> String {
    let duration = ffprobe
        .get("format")
        .and_then(|format_info| format_info.get("duration"))
        .and_then(serde_json::Value::as_str)
        .and_then(|raw| raw.parse::<f64>().ok());
    match duration {
        Some(duration) if duration.is_finite() && duration >= 0.0 => {
            let total_seconds = duration as i64;
            let hours = total_seconds / 3600;
            let minutes = (total_seconds % 3600) / 60;
            let seconds = total_seconds % 60;
            format!("{hours:02}H:{minutes:02}M:{seconds:02}S")
        }
        _ => "NA".to_string(),
    }
}

pub async fn user_media_movie_detail(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(guid): Path<uuid::Uuid>,
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
        // the detail guid is a media guid (mm_media_guid); resolve the movie
        // metadata it belongs to
        let metadata_guid = match sqlx::query_scalar::<_, uuid::Uuid>(
            r#"select mm_media_metadata_guid
                   from mm_media
                  where mm_media_guid = $1
                  limit 1"#,
        )
        .bind(guid)
        .fetch_optional(&state.sqlx_pool_ro)
        .await
        {
            Ok(Some(metadata_guid)) => metadata_guid,
            Ok(None) => {
                return (
                    StatusCode::NOT_FOUND,
                    Html(String::from("Not found")).into_response(),
                );
            }
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(String::from("Internal server error")).into_response(),
                );
            }
        };
        let movie_metadata = match mk_lib_database::database_metadata::mk_lib_database_metadata_movie::mk_lib_database_metadata_movie_detail_by_guid(
            &state.sqlx_pool_ro,
            metadata_guid,
            current_user.id,
        )
        .await
        {
            Ok(row) => row,
            Err(sqlx::Error::RowNotFound) => {
                return (
                    StatusCode::NOT_FOUND,
                    Html(String::from("Not found")).into_response(),
                );
            }
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(String::from("Internal server error")).into_response(),
                );
            }
        };

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
        let request_status: serde_json::Value = json!(false);
        let rating_status: serde_json::Value = json!(null);
        let queue_status: serde_json::Value = json!(false);

        let mut mm_poster: String = "/static/image/Movie-icon.png".to_string();
        if !movie_metadata.mm_poster.is_empty() {
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

        let (cast_members, crew_members) =
            movie_credits(&state.sqlx_pool_ro, &movie_metadata.mm_metadata_movie_json).await;

        // every media file (version) of this movie
        let file_rows = sqlx::query_as::<_, MediaMovieFileRow>(
            r#"select mm_media_guid,
                      mm_media_path,
                      mm_media_ffprobe_json
               from mm_media
               where mm_media_metadata_guid = $1
               order by mm_media_path"#,
        )
        .bind(metadata_guid)
        .fetch_all(&state.sqlx_pool_ro)
        .await;
        let file_rows = match file_rows {
            Ok(rows) => rows,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(String::from("Internal server error")).into_response(),
                );
            }
        };
        let mut template_media_files: Vec<TemplateMediaFileList> = Vec::new();
        for file_row in file_rows {
            let file_name = file_row
                .mm_media_path
                .rsplit('/')
                .next()
                .unwrap_or(&file_row.mm_media_path)
                .to_string();
            let (resolution, runtime) = match &file_row.mm_media_ffprobe_json {
                Some(ffprobe) => (
                    parse_ffprobe_resolution(ffprobe),
                    parse_ffprobe_runtime(ffprobe),
                ),
                // not all files have ffprobe data
                None => ("NA".to_string(), "NA".to_string()),
            };
            template_media_files.push(TemplateMediaFileList {
                mm_media_guid: file_row.mm_media_guid,
                file_name,
                resolution,
                runtime,
            });
        }

        let template = TemplateMediaMovieDetailContext {
            template_data_json: &movie_metadata.mm_metadata_movie_json,
            template_metadata_guid: movie_metadata.mm_metadata_movie_guid,
            template_metadata_name_alt: movie_metadata.mm_metadata_movie_name_alt.clone(),
            template_metadata_poster: mm_poster,
            template_metadata_backdrop: "/static/image/Movie-icon.png".to_string(),
            template_metadata_rating: "pg13".to_string(),
            template_metadata_star_rating: movie_metadata.mm_metadata_vote_average.unwrap_or(0.0)
                as f32,
            template_metadata_availability: "available".to_string(),
            template_metadata_tagline: movie_metadata.mm_metadata_tagline.clone(),
            template_metadata_photo_updated: movie_metadata.photo_updated,
            template_metadata_genre: genres,
            template_metadata_cast: cast_members,
            template_metadata_crew: crew_members,
            template_metadata_user_watched: watched_status,
            template_metadata_user_rating: rating_status,
            template_metadata_user_request: request_status,
            template_metadata_user_queue: queue_status,
            template_media_files,
            page_title: Some("MediaKraken Movie Detail".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

pub async fn user_media_movie_status(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    axum::Json(payload): axum::Json<mk_lib_database::mk_lib_database::MediaStatusUpdatePayload>,
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
        StatusCode::UNAUTHORIZED.into_response()
    } else {
        mk_lib_database::database_metadata::mk_lib_database_metadata_movie::mk_lib_database_metadata_movie_status(
            &state.sqlx_pool_rw, payload, current_user.id
        )
        .await
        .unwrap();
        StatusCode::OK.into_response()
    }
}

/*
   """
   Display move detail page
   """
   db_connection = await request.app.db_pool.acquire()
   if request.method == 'POST':
       if request.form['playback'] == 'Web Viewer':
           common_network_pika.com_net_pika_send(
               {'Type': 'Playback', 'Subtype': 'Play', 'Device': 'Web',
                'User': user.id,
                'Data': await
                request.app.db_functions.db_read_media(guid, db_connection=db_connection)[
                    'mm_media_path']},
               rabbit_host_name='mkstack-rabbitmq',
               exchange_name='mkque_ex',
               route_key='mkque')
           return redirect(
               request.app.url_for('name_blueprint_user_playback_video.user_video_player_videojs',
                                   mtype='hls',
                                   guid=request.form['Video_Track'], chapter=1,
                                   audio=request.form['Video_Play_Audio_Track'],
                                   sub=request.form['Video_Play_Subtitles']))
       if request.form['status'] == 'Watched':
           await request.app.db_functions.db_meta_movie_status_update(guid, user.id, False,
                                                                      db_connection=db_connection)
           return redirect(
               request.app.url_for('name_blueprint_user_movie.url_bp_user_movie_detail',
                                   guid=guid))
       else if request.form['status'] == 'Unwatched':
           await request.app.db_functions.db_meta_movie_status_update(guid, user.id, True,
                                                                      db_connection=db_connection)
           return redirect(
               request.app.url_for('name_blueprint_user_movie.url_bp_user_movie_detail',
                                   guid=guid))
       else if request.form['status'] == 'Sync':
           return redirect(
               request.app.url_for('name_blueprint_user_sync.url_bp_user_sync_edit', guid=guid))
       else if request.form['status'] == 'Cast':
           // TODO submit cast comment via rabbitmq
           # grab the guid from the comboindex
           media_guid_index = request.form["Video_Track"]
           # call ffpmeg with the play_data
           audio_track_index = request.form["Video_Play_Audio_Track"]
           subtitle_track_index = request.form["Video_Play_Subtitles"]
           # launch ffmpeg to ffserver procecss
           // TODO fire up node cast container!
           proc_ffserver = subprocess.Popen(split('ffmpeg  -i \"',
                                                  await
                                                  request.app.db_functions.db_media_path_by_uuid(
                                                      media_guid_index,
                                                      db_connection=db_connection)[
                                                      0] + '\" http://localhost/stream.ffm'),
                                            stdout=subprocess.PIPE, shell=False)
           await mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(message_type='info',
                                                                            message_text={
                                                                                "FFServer PID": proc_ffserver.pid})
           return redirect(
               request.app.url_for('name_blueprint_user_movie.url_bp_user_movie_detail',
                                   guid=guid))
   else:
       metadata_data = await request.app.db_functions.db_meta_movie_by_media_uuid(guid,
                                                                                  db_connection=db_connection)
       # poster image
       try:
           # don't bother checking for NONE as that's valid
           data_poster_image = metadata_data['mm_metadata_localimage_json']['Poster']
       except:
           data_poster_image = None
       # background image
       try:
           # don't bother checking for NONE as that's valid
           data_background_image = metadata_data['mm_metadata_localimage_json']['Backdrop']
       except:
           data_background_image = None

       # build gen list
       genres_list = []
       for ndx in range(0, len(metadata_data['mm_metadata_json']['genres'])):
           genres_list.append(metadata_data['mm_metadata_json']['genres'][ndx]['name'])

       # not sure if the following with display anymore
       # # vote count format
       # data_vote_count = common_internationalization.com_inter_number_format(
       #     metadata_data['mm_metadata_json']['vote_count'])
       # # build production list
       # production_list = ''
       # for ndx in range(0,
       #                  len(metadata_data['mm_metadata_json']['production_companies'])):
       #     production_list \
       #         += (metadata_data['mm_metadata_json']['production_companies'][ndx]['name']
       #             + ', ')
       # # budget format
       # budget = common_internationalization.com_inter_number_format(
       #     metadata_data['mm_metadata_json']['budget'])
       # # revenue format
       # revenue = common_internationalization.com_inter_number_format(
       #     metadata_data['mm_metadata_json']['revenue'])
       # # grab reviews
       # review = []
       # review_json = await request.app.db_functions.db_review_list_by_tmdb_guid(json_metaid['themoviedb'], db_connection)
       # if review_json != None and len(review_json) > 0:
       #     review_json = review_json[0]
       #     for review_data in review_json[1]['themoviedb']['results']:
       #         review.append(
       #             (review_data['author'], review_data['url'], review_data['content']))
       # # set watched and sync
       # try:
       #     watched_status = json_media['UserStats'][user.id]['Watched']
       # except:
       #     watched_status = False
       # try:
       #     sync_status = json_media['Synced']
       # except:
       #     sync_status = False

       # check to see if there are other version(s) of this video file (dvd, hddvd, etc)
       ffprobe_data = {}
       // TODO  the following does alot of repeats sumhow.   due to dict it stomps over itself
       for video_version in await request.app.db_functions.db_media_ffprobe_all_guid(guid,
                                                                                     common_global.DLMediaType.Movie.value,
                                                                                     db_connection=db_connection):
           await mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_push(message_type='info',
                                                                            message_text={
                                                                                "vid_version": video_version})
           # not all files have ffprobe
           if video_version['mm_media_ffprobe_json'] == None:
               hours = 0
               minutes = 0
               seconds = 0
               data_resolution = "NA"
           else:
               # calculate a better runtime instead of just seconds
               try:
                   minutes, seconds = divmod(
                       float(video_version['mm_media_ffprobe_json']['format']['duration']), 60)
                   hours, minutes = divmod(minutes, 60)
               except:
                   hours = 0
                   minutes = 0
                   seconds = 0
               // TODO will need to be able to loop through streams...for those with multiple videos in container
               try:
                   data_resolution = str(
                       video_version['mm_media_ffprobe_json']['streams'][0]['width']) + 'x' \
                                     + str(
                       video_version['mm_media_ffprobe_json']['streams'][0]['height'])
               except:
                   data_resolution = 'NA'
           # audio and sub streams
           audio_streams = []
           subtitle_streams = []
           if video_version['mm_media_ffprobe_json'] != None:
               for stream_info in video_version['mm_media_ffprobe_json']['streams']:
                   if stream_info['codec_type'] == 'audio':
                       try:
                           stream_language = common_internationalization.com_inter_country_name(
                               stream_info['tags']['language'])  # eng, spa and so on
                       except KeyError:
                           stream_language = 'NA'
                       try:
                           stream_title = stream_info['tags']['title']  # Surround 5.1 and so on
                       except KeyError:
                           stream_title = 'NA'
                       if 'codec_long_name' in stream_info and stream_info[
                           'codec_long_name'] != 'unknown':
                           stream_codec = stream_info['codec_long_name']
                       else:
                           try:
                               stream_codec = stream_info['codec_name']
                           except KeyError:
                               stream_codec = 'NA'
                       audio_streams.append((stream_codec, stream_language, stream_title))
                   else if stream_info['codec_type'] == 'subtitle':
                       try:
                           subtitle_streams.append(
                               common_internationalization.com_inter_country_name(
                                   stream_info['tags']['language']))
                       except KeyError:
                           subtitle_streams.append('Unknown')
           ffprobe_data[video_version['mm_media_guid']] = (data_resolution,
                                                           "%02dH:%02dM:%02dS" % (
                                                               hours, minutes, seconds),
                                                           audio_streams,
                                                           subtitle_streams)
       # do chapter stuff here so I can sort
       data_json_media_chapters = []
       # try:
       #     for chap_data in natsort.natsorted(json_media['ChapterImages']):
       #         data_json_media_chapters.append((chap_data,
       #                                          json_media['ChapterImages'][chap_data]))
       # except:
       #     pass

       # find all devices to playback media on
       playback_devices = []
       for device_item in await request.app.db_functions.db_device_list(
               db_connection=db_connection):
           if device_item['mm_device_type'] == 'Chromecast':
               playback_devices.append(device_item['mm_device_json']['Name'])
           else if device_item['mm_device_type'] == 'Roku':
               playback_devices.append(device_item)

       await request.app.db_pool.release(db_connection)
       return {
           'json_metadata': metadata_data,
           'data_genres': genres_list,
           'data_poster_image': data_poster_image,
           'data_background_image': data_background_image,
           'data_json_media_chapters': data_json_media_chapters,
           'data_ffprobe_data': ffprobe_data,
           'data_playback_device': playback_devices,
           # data_watched_status=watched_status,
           # data_sync_status=sync_status
       }

*/
