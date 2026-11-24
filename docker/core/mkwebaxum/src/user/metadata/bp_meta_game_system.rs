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

/// One game system row on the metadata game system list page.
///
/// The shared-lib list/count queries use a pgroonga search operator and
/// an outdated `gs_id` column name, so the list lives here (same
/// approach as the media movie list).
#[derive(Debug, sqlx::FromRow)]
struct MetaGameSystemListRow {
    gs_game_system_id: uuid::Uuid,
    gs_game_system_name: String,
    gs_year: Option<String>,
}

/// Count game systems using the list filter (`starts_with` is "", "#",
/// or a single letter).
async fn meta_game_system_count(sqlx_pool: &PgPool, starts_with: &str) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        r#"select count(*)
           from mm_metadata_game_systems_info
           where (
                $1 = ''
                or ($1 = '#' and left(lower(gs_game_system_name), 1) !~ '^[a-z0-9]$')
                or ($1 <> '#' and lower(gs_game_system_name) like lower($1) || '%')
              )"#,
    )
    .bind(starts_with)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

/// Read game systems using the list filter, name ordered, paginated.
async fn meta_game_system_read(
    sqlx_pool: &PgPool,
    starts_with: &str,
    offset: i64,
    limit: i64,
) -> Result<Vec<MetaGameSystemListRow>, sqlx::Error> {
    let rows: Vec<MetaGameSystemListRow> = sqlx::query_as(
        r#"select gs_game_system_id,
           coalesce(gs_game_system_name, '') as gs_game_system_name,
           gs_game_system_json->>'year' as gs_year
           from mm_metadata_game_systems_info
           where (
                $1 = ''
                or ($1 = '#' and left(lower(gs_game_system_name), 1) !~ '^[a-z0-9]$')
                or ($1 <> '#' and lower(gs_game_system_name) like lower($1) || '%')
              )
           order by lower(gs_game_system_name), gs_game_system_id
           offset $2 limit $3"#,
    )
    .bind(starts_with)
    .bind(offset)
    .bind(limit)
    .fetch_all(sqlx_pool)
    .await?;
    Ok(rows)
}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_game_system.html")]
struct TemplateMetaGameSystemContext<'a> {
    template_data: &'a Vec<MetaGameSystemListRow>,
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

pub async fn user_metadata_game_system(
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
        let total_items: i64 =
            match meta_game_system_count(&state.sqlx_pool_ro, &starts_with_value).await {
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
            "/user/metadata/game_system".to_string(),
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
        let game_system_list = match meta_game_system_read(
            &state.sqlx_pool_ro,
            &starts_with_value,
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
        let mut template_data_exists = false;
        if !game_system_list.is_empty() {
            template_data_exists = true;
        }
        let page_usize = page as usize;
        let template = TemplateMetaGameSystemContext {
            template_data: &game_system_list,
            template_data_exists: &template_data_exists,
            pagination_bar: &pagination_html,
            #[allow(dead_code)]
            page: &page_usize,
            page_title: Some("MediaKraken Metadata Game Systems".to_string()),
            current: starts_with,
            genre_filter: None,
            genre_filter_query: None,
            primary_language_filter: None,
            status_filter: None,
            base_path: "/user/metadata/game_system".to_string(),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

#[derive(Template)]
#[template(path = "bss_user/metadata/bss_user_metadata_game_system_detail.html")]
struct TemplateMetaGameSystemDetailContext {
    template_data: serde_json::Value,
    page_title: Option<String>,
}

pub async fn user_metadata_game_system_detail(
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
        // The shared-lib detail query references the pre-rename `gs_id`
        // column, so the query lives here (same approach as the media
        // movie list).
        let detail_data: serde_json::Value = match sqlx::query_as(
            r#"select coalesce(gs_game_system_json, '{}'::jsonb)
                   from mm_metadata_game_systems_info
                   where gs_game_system_id = $1"#,
        )
        .bind(guid)
        .fetch_optional(&state.sqlx_pool_ro)
        .await
        {
            Ok(Some((data,))) => data,
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
        let template = TemplateMetaGameSystemDetailContext {
            template_data: crate::user::detail_map::game_system(&detail_data),
            page_title: Some("MediaKraken Metadata Game System Detail".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

/*
@blueprint_user_metadata_game_system.route('/user_meta_game_system', methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/metadata/bss_user_metadata_game_system.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_game_system(request):
    """
    Display list of game system metadata
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    request.ctx.session['search_page'] = 'meta_game_system'
    db_connection = await request.app.db_pool.acquire()
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_meta_game',
                                                                      item_count=await request.app.db_functions.db_meta_game_system_list_count(
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    media_data = await request.app.db_functions.db_meta_game_system_list(offset,
                                                                         int(request.ctx.session[
                                                                                 'per_page']),
                                                                         request.ctx.session[
                                                                             'search_text'],
                                                                         db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'media': media_data,
        'pagination_bar': pagination,
    }


@blueprint_user_metadata_game_system.route('/user_meta_game_system_detail/<guid>')
@common_global.jinja_template.template(
    'bss_user/metadata/bss_user_metadata_game_system_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_metadata_game_system_detail(request, guid):
    """
    Display metadata game detail
    """
    db_connection = await request.app.db_pool.acquire()
    media_data = await request.app.db_functions.db_meta_game_system_by_guid(guid,
                                                                            db_connection=db_connection)
    await request.app.db_pool.release(db_connection)
    return {
        'guid': guid,
        'data': media_data,
    }

 */
