use crate::mk_lib_database;
use crate::user_preferences;
use crate::AppState;
use askama::Template;
use axum::extract::Query;
use axum::extract::State;
use axum::response::Redirect;
use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    Extension,
};
use axum_session::{SessionConfig, SessionLayer};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use core::fmt::Write;
use paginator::{PageItem, Paginator};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_game.html")]
struct TemplateMetaGameContext<'a> {
    template_data:
        &'a Vec<mk_lib_database::database_metadata::mk_lib_database_metadata_game::DBMetaGameList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
    page_title: Option<String>,
    pub current: Option<String>,
    pub genre_filter: Option<String>,
    pub genre_filter_query: Option<String>,
    pub status_filter: Option<String>,
    pub status_options: &'a Vec<FilterOption>,
    pub genre_options: &'a Vec<FilterOption>,
    pub base_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FilterOption {
    pub label: String,
    pub query_value: String,
}

#[derive(Debug, Deserialize)]
pub struct FilterQuery {
    pub starts_with: Option<String>,
    pub genre: Option<String>,
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
    if let Some(status) = status_filter.filter(|status| !status.is_empty()) {
        query_params.push(format!("status={}", urlencoding::encode(status)));
    }
    if query_params.is_empty() {
        String::new()
    } else {
        format!("?{}", query_params.join("&"))
    }
}

fn build_game_pagination(
    total_items: i64,
    page: i64,
    pagination_count: i64,
    starts_with: Option<&str>,
    genre: Option<&str>,
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

    let suffix = build_filter_query_suffix(starts_with, genre, status_filter);
    let current_page = page.max(1).min(total_pages) as usize;
    let paginator = Paginator::builder(total_pages as usize)
        .current_page(current_page)
        .build_paginator()
        .map_err(|_| std::fmt::Error)?;

    for item in paginator.paginate() {
        match item {
            PageItem::Prev(p) => {
                write!(
                    pagination_html,
                    r#"<li><a href="/user/metadata/game/{p}{suffix}"
class="px-3 py-2 rounded-md border border-gray-300 text-gray-700 hover:bg-gray-200"
aria-label="Previous">&laquo;</a></li>"#,
                )?;
            }
            PageItem::Page(p) => {
                write!(
                    pagination_html,
                    r#"<li><a href="/user/metadata/game/{p}{suffix}"
class="px-3 py-2 rounded-md border border-gray-300 text-gray-700 hover:bg-gray-200">{p}</a></li>"#,
                )?;
            }
            PageItem::CurrentPage(p) => {
                write!(
                    pagination_html,
                    r#"<li><a href="/user/metadata/game/{p}{suffix}"
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
                    r#"<li><a href="/user/metadata/game/{p}{suffix}"
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

pub async fn user_metadata_game(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(page): Path<i64>,
    Query(params): Query<FilterQuery>,
) -> impl IntoResponse {
    let starts_with = normalize_starts_with(params.starts_with.as_deref());
    let genre = normalize_string_filter(params.genre.as_deref());
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
        let total_pages: i64 =
        mk_lib_database::database_metadata::mk_lib_database_metadata_game::mk_lib_database_metadata_game_count(
           &state.sqlx_pool_ro,
            String::new(),
            starts_with.clone().unwrap_or_default(),
            genre.clone().unwrap_or_default(),
            status_filter.clone().unwrap_or_default(),
        )
        .await
        .unwrap();
        let pagination_html = build_game_pagination(
            total_pages,
            page,
            pagination_count,
            starts_with.as_deref(),
            genre.as_deref(),
            status_filter.as_deref(),
        )
        .unwrap();
        let game_list =
        mk_lib_database::database_metadata::mk_lib_database_metadata_game::mk_lib_database_metadata_game_read(
           &state.sqlx_pool_ro,
            String::new(),
            starts_with.clone().unwrap_or_default(),
            genre.clone().unwrap_or_default(),
            status_filter.clone().unwrap_or_default(),
            db_offset,
            pagination_count,
        )
        .await
        .unwrap();
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
        let genre_options: Vec<FilterOption> =
            mk_lib_database::database_metadata::mk_lib_database_metadata_game::mk_lib_database_metadata_game_genres(
                &state.sqlx_pool_ro,
            )
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|value| FilterOption {
                label: value.clone(),
                query_value: value,
            })
            .collect();
        let mut template_data_exists = false;
        if game_list.len() > 0 {
            template_data_exists = true;
        }
        let page_usize = page as usize;
        let template = TemplateMetaGameContext {
            template_data: &game_list,
            template_data_exists: &template_data_exists,
            pagination_bar: &pagination_html,
            page: &page_usize,
            page_title: Some("MediaKraken Metadata Games".to_string()),
            current: starts_with.clone(),
            genre_filter: genre.clone(),
            genre_filter_query: genre
                .as_ref()
                .map(|value| urlencoding::encode(value).into_owned()),
            status_filter: status_filter.clone(),
            status_options: &status_options,
            genre_options: &genre_options,
            base_path: "/user/metadata/game".to_string(),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_game_detail.html")]
struct TemplateMetaGameDetailContext {
    template_data: serde_json::Value,
    page_title: Option<String>,
}

pub async fn user_metadata_game_detail(
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
        let template = TemplateMetaGameDetailContext {
            template_data: json!({}),
            page_title: Some("MediaKraken Metadata Game Detail".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

/*
@blueprint_user_metadata_game.route('/user_meta_game', methods=["GET", "POST"])
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_game.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_game(request):
    """
    Display game list metadata
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    request.ctx.session['search_page'] = 'meta_game'
    db_connection = await request.app.db_pool.acquire()
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_meta_game',
                                                                      item_count=await request.app.db_functions.db_table_count(
                                                                          table_name='mm_metadata_game_software_info',
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    media_data = await request.app.db_functions.db_meta_game_list(offset,
                                                                  int(request.ctx.session[
                                                                          'per_page']),
                                                                  request.ctx.session[
                                                                      'search_text'],
                                                                  db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'media_game': media_data,
        'pagination_bar': pagination,
    }


@blueprint_user_metadata_game.route('/user_meta_game_detail/<guid>')
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_game_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_game_detail(request, guid):
    """
    Display game metadata detail
    """
    db_connection = await request.app.db_pool.acquire()
    media_data = await \
        request.app.db_functions.db_meta_game_by_guid(guid, db_connection=db_connection)[
            'gi_game_info_json']
    await request.app.db_pool.release(db_connection)
    return {
        'guid': guid,
        'data': media_data,
        'data_review': None,
    }

 */
