use crate::AppState;
use crate::mk_lib_database;
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
use mk_lib_common::mk_lib_common_pagination;
use serde::Deserialize;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_music.html")]
struct TemplateMetaMusicBandListContext<'a> {
    template_data: &'a Vec<
        mk_lib_database::database_metadata::mk_lib_database_metadata_musicbrainz::DBMetaMusicBrainzBand,
    >,
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
    pub base_path: String,
}

#[derive(Debug, Deserialize)]
pub struct FilterQuery {
    pub starts_with: Option<String>,
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

/// Band list: artist credits that have at least one release, with a letter
/// filter and pagination.
pub async fn user_metadata_music(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(page): Path<i64>,
    Query(params): Query<FilterQuery>,
) -> impl IntoResponse {
    let starts_with = normalize_starts_with(params.starts_with.as_deref());
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
        let starts_with_value = starts_with.clone().unwrap_or_default();
        let total_items =
            mk_lib_database::database_metadata::mk_lib_database_metadata_musicbrainz::mk_lib_database_metadata_musicbrainz_band_count(
                &state.sqlx_pool_ro,
                &starts_with_value,
            )
            .await;
        let total_items = match total_items {
            Ok(total) => total,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(String::from("Internal server error")).into_response(),
                );
            }
        };
        let pagination_html = match mk_lib_common_pagination::mk_lib_common_paginate(
            total_items,
            page,
            "/user/metadata/music".to_string(),
            starts_with.as_deref(),
            pagination_count,
        )
        .await
        {
            Ok(html) => html,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(String::from("Internal server error")).into_response(),
                );
            }
        };
        let band_list = mk_lib_database::database_metadata::mk_lib_database_metadata_musicbrainz::mk_lib_database_metadata_musicbrainz_band_read(
            &state.sqlx_pool_ro,
            &starts_with_value,
            db_offset,
            pagination_count,
        )
        .await;
        let band_list = match band_list {
            Ok(list) => list,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(String::from("Internal server error")).into_response(),
                );
            }
        };
        let mut template_data_exists = false;
        if !band_list.is_empty() {
            template_data_exists = true;
        }
        let page_usize = page as usize;
        let template = TemplateMetaMusicBandListContext {
            template_data: &band_list,
            template_data_exists: &template_data_exists,
            pagination_bar: &pagination_html,
            #[allow(dead_code)]
            page: &page_usize,
            page_title: Some("MediaKraken Metadata Music".to_string()),
            current: starts_with,
            genre_filter: None,
            genre_filter_query: None,
            primary_language_filter: None,
            status_filter: None,
            base_path: "/user/metadata/music".to_string(),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_music_band.html")]
struct TemplateMetaMusicBandContext<'a> {
    band: &'a mk_lib_database::database_metadata::mk_lib_database_metadata_musicbrainz::DBMetaMusicBrainzBand,
    template_data: &'a Vec<
        mk_lib_database::database_metadata::mk_lib_database_metadata_musicbrainz::DBMetaMusicBrainzAlbum,
    >,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    #[allow(dead_code)]
    page: &'a usize,
    page_title: Option<String>,
}

/// Band detail: one artist credit's releases ("albums"), paginated.
pub async fn user_metadata_music_band(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path((credit_id, page)): Path<(i32, i64)>,
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
        let band =
            match mk_lib_database::database_metadata::mk_lib_database_metadata_musicbrainz::mk_lib_database_metadata_musicbrainz_band(
                &state.sqlx_pool_ro,
                credit_id,
            )
            .await
            {
                Ok(Some(band)) => band,
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
        let pagination_count =
            user_preferences::load_user_pagination_count(&state.sqlx_pool_ro, current_user.id)
                .await
                .unwrap_or(user_preferences::DEFAULT_PAGINATION_COUNT);
        let db_offset: i64 = (page * pagination_count) - pagination_count;
        let total_items =
            mk_lib_database::database_metadata::mk_lib_database_metadata_musicbrainz::mk_lib_database_metadata_musicbrainz_album_count(
                &state.sqlx_pool_ro,
                credit_id,
            )
            .await;
        let total_items = match total_items {
            Ok(total) => total,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(String::from("Internal server error")).into_response(),
                );
            }
        };
        let pagination_html = match mk_lib_common_pagination::mk_lib_common_paginate(
            total_items,
            page,
            format!("/user/metadata/music_band/{credit_id}"),
            None,
            pagination_count,
        )
        .await
        {
            Ok(html) => html,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(String::from("Internal server error")).into_response(),
                );
            }
        };
        let album_list =
            mk_lib_database::database_metadata::mk_lib_database_metadata_musicbrainz::mk_lib_database_metadata_musicbrainz_album_read(
                &state.sqlx_pool_ro,
                credit_id,
                db_offset,
                pagination_count,
            )
            .await;
        let album_list = match album_list {
            Ok(list) => list,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(String::from("Internal server error")).into_response(),
                );
            }
        };
        let mut template_data_exists = false;
        if !album_list.is_empty() {
            template_data_exists = true;
        }
        let page_usize = page as usize;
        let template = TemplateMetaMusicBandContext {
            band: &band,
            template_data: &album_list,
            template_data_exists: &template_data_exists,
            pagination_bar: &pagination_html,
            #[allow(dead_code)]
            page: &page_usize,
            page_title: Some(format!("MediaKraken Metadata: {}", band.mb_band_name)),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

/// One rendered tracklist row; `medium_label`/`new_medium` drive the
/// "Disc N" separator rows (only the first track of each medium has
/// `new_medium` set).
#[derive(Debug)]
struct TemplateMetaMusicAlbumTrackRow {
    medium_label: String,
    new_medium: bool,
    number: String,
    name: String,
    artist: String,
    length: String,
}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_music_album.html")]
struct TemplateMetaMusicAlbumContext {
    album: mk_lib_database::database_metadata::mk_lib_database_metadata_musicbrainz::DBMetaMusicBrainzAlbumDetail,
    tracks: Vec<TemplateMetaMusicAlbumTrackRow>,
    track_count: i64,
    total_runtime: String,
    page_title: Option<String>,
}

/// Track length in ms to `m:ss` ("" when unknown).
fn format_track_length(ms: Option<i32>) -> String {
    let Some(ms) = ms.filter(|ms| *ms > 0) else {
        return String::new();
    };
    let total_seconds = (ms / 1000) as i64;
    format!("{}:{:02}", total_seconds / 60, total_seconds % 60)
}

/// Total album runtime to `m:ss` (or `h:mm:ss` past one hour), "" when
/// every track lacks a length.
fn format_total_runtime(total_ms: i64) -> String {
    if total_ms <= 0 {
        return String::new();
    }
    let total_seconds = total_ms / 1000;
    if total_seconds >= 3600 {
        format!(
            "{}:{:02}:{:02}",
            total_seconds / 3600,
            (total_seconds % 3600) / 60,
            total_seconds % 60
        )
    } else {
        format!("{}:{:02}", total_seconds / 60, total_seconds % 60)
    }
}

/// Album detail: release header plus its full tracklist.
pub async fn user_metadata_music_album(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(release_id): Path<i32>,
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
        let album =
            match mk_lib_database::database_metadata::mk_lib_database_metadata_musicbrainz::mk_lib_database_metadata_musicbrainz_album(
                &state.sqlx_pool_ro,
                release_id,
            )
            .await
            {
                Ok(Some(album)) => album,
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
        let track_rows =
            mk_lib_database::database_metadata::mk_lib_database_metadata_musicbrainz::mk_lib_database_metadata_musicbrainz_album_tracks(
                &state.sqlx_pool_ro,
                release_id,
            )
            .await;
        let track_rows = match track_rows {
            Ok(rows) => rows,
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(String::from("Internal server error")).into_response(),
                );
            }
        };

        // A track's artist is only shown when it differs from the album's
        // artist credit phrase (guest/featured artists).
        let credit_phrase = album.mb_album_band_name.clone();
        let mut tracks = Vec::with_capacity(track_rows.len());
        let mut total_ms = 0i64;
        let mut last_medium = 0i32;
        for track in track_rows {
            let new_medium = track.mb_medium_position != last_medium;
            last_medium = track.mb_medium_position;
            if let Some(length) = track.mb_track_length_ms {
                total_ms += length as i64;
            }
            let artist =
                if track.mb_track_artist.is_empty() || track.mb_track_artist == credit_phrase {
                    String::new()
                } else {
                    track.mb_track_artist
                };
            let medium_label = if new_medium {
                let mut label = format!("Disc {}", track.mb_medium_position);
                if !track.mb_medium_format.is_empty() {
                    label.push_str(&format!(" · {}", track.mb_medium_format));
                }
                label
            } else {
                String::new()
            };
            tracks.push(TemplateMetaMusicAlbumTrackRow {
                medium_label,
                new_medium,
                number: track.mb_track_number,
                name: track.mb_track_name,
                artist,
                length: format_track_length(track.mb_track_length_ms),
            });
        }

        let template = TemplateMetaMusicAlbumContext {
            album,
            track_count: tracks.len() as i64,
            total_runtime: format_total_runtime(total_ms),
            tracks,
            page_title: Some("MediaKraken Metadata Music Album".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}
