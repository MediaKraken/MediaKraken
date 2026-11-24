use crate::AppState;
use crate::mk_lib_database;
use crate::user_preferences;
use askama::Template;
use axum::extract::State;
use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use mk_lib_common::mk_lib_common_enum_media_type::DLMediaType;
use mk_lib_common::mk_lib_common_pagination;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_401.html")]
struct TemplateError401Context {}

/// One sports metadata row on the sports list page (local media only).
#[derive(Debug, sqlx::FromRow)]
struct MediaSportsListRow {
    mm_metadata_sports_guid: uuid::Uuid,
    mm_metadata_sports_name: String,
}

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_sports.html")]
struct TemplateMediaSportsContext<'a> {
    template_data: &'a Vec<MediaSportsListRow>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    #[allow(dead_code)]
    page: &'a usize,
    page_title: Option<String>,
}

/// Count the sports metadata titles that have local media.
///
/// The shared lib `mk_lib_database_media_sports_count` is an empty-SQL stub
/// in the deployed registry crate, so the query lives here (same approach as
/// the media movie list).
async fn media_sports_count(sqlx_pool: &PgPool) -> Result<i64, sqlx::Error> {
    let row: (i64,) = sqlx::query_as(
        r#"select count(distinct mm_metadata_sports_guid)
           from mm_media, mm_metadata_sports
           where mm_media_class_enum = $1
           and mm_media_metadata_guid = mm_metadata_sports_guid"#,
    )
    .bind(DLMediaType::SPORTS)
    .fetch_one(sqlx_pool)
    .await?;
    Ok(row.0)
}

/// Read one representative sports metadata row per title, name ordered.
///
/// The shared lib `mk_lib_database_media_sports_read` is an empty-SQL stub
/// in the deployed registry crate, so the query lives here (same approach as
/// the media movie list).
async fn media_sports_read(
    sqlx_pool: &PgPool,
    offset: i64,
    limit: i64,
) -> Result<Vec<MediaSportsListRow>, sqlx::Error> {
    let rows: Vec<MediaSportsListRow> = sqlx::query_as(
        r#"select temp.mm_metadata_sports_guid,
           coalesce(mm_metadata_sports_name, '') as mm_metadata_sports_name
           from (select distinct on (mm_metadata_sports_guid) mm_metadata_sports_guid
                 from mm_media, mm_metadata_sports
                 where mm_media_class_enum = $1
                 and mm_media_metadata_guid = mm_metadata_sports_guid
                 order by mm_metadata_sports_guid, mm_media_path) as temp
           join mm_metadata_sports
             on mm_metadata_sports.mm_metadata_sports_guid = temp.mm_metadata_sports_guid
           order by LOWER(mm_metadata_sports_name), mm_metadata_sports_guid
           offset $2 limit $3"#,
    )
    .bind(DLMediaType::SPORTS)
    .bind(offset)
    .bind(limit)
    .fetch_all(sqlx_pool)
    .await?;
    Ok(rows)
}

pub async fn user_media_sports(
    State(state): State<AppState>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(page): Path<i64>,
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
        let pagination_count =
            user_preferences::load_user_pagination_count(&state.sqlx_pool_ro, current_user.id)
                .await
                .unwrap_or(user_preferences::DEFAULT_PAGINATION_COUNT);
        let db_offset: i64 = (page * pagination_count) - pagination_count;
        let total_items: i64 = match media_sports_count(&state.sqlx_pool_ro).await {
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
            "/user/media/sports".to_string(),
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
        let sports_list =
            match media_sports_read(&state.sqlx_pool_ro, db_offset, pagination_count).await {
                Ok(list) => list,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Html(String::from("Internal server error")).into_response(),
                    );
                }
            };
        let mut template_data_exists = false;
        if !sports_list.is_empty() {
            template_data_exists = true;
        }
        let page_usize = page as usize;
        let template = TemplateMediaSportsContext {
            template_data: &sports_list,
            template_data_exists: &template_data_exists,
            pagination_bar: &pagination_html,
            #[allow(dead_code)]
            page: &page_usize,
            page_title: Some("MediaKraken Sports".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

#[derive(sqlx::FromRow)]
struct MediaSportsDetailRow {
    mm_metadata_sports_json: serde_json::Value,
    cover_path: String,
}

#[derive(Template)]
#[template(path = "bss_user/media/bss_user_media_sports_detail.html")]
struct TemplateMediaSportsDetailContext {
    template_data: serde_json::Value,
    page_title: Option<String>,
}

pub async fn user_media_sports_detail(
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
        let row = sqlx::query_as::<_, MediaSportsDetailRow>(
            r#"select mm_metadata_sports_json,
                      COALESCE(mm_metadata_sports_image_json->'Images'->'thesportsdb'->>'Poster',
                               mm_metadata_sports_image_json->>'Poster', '') as cover_path
               from mm_metadata_sports
               where mm_metadata_sports_guid = $1"#,
        )
        .bind(guid)
        .fetch_optional(&state.sqlx_pool_ro)
        .await;
        let template_data = match row {
            Ok(Some(row)) => {
                crate::user::detail_map::sports(&row.mm_metadata_sports_json, &row.cover_path)
            }
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
        let template = TemplateMediaSportsDetailContext {
            template_data,
            page_title: Some("MediaKraken Sports Detail".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

/*
# list of spoting events
@blueprint_user_sports.route("/user_sports", methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_sports.html')
@common_global.auth.login_required
pub async fn url_bp_user_sports(request):
    """
    Display sporting events page
    """
    page, offset = common_pagination_bootstrap.com_pagination_page_calc(request)
    media = []
    db_connection = await request.app.db_pool.acquire()
    for row_data in await request.app.db_functions.db_media_sports_list(
            common_global.DLMediaType.Sports.value,
            offset,
            int(request.ctx.session[
                    'per_page']),
            request.ctx.session[
                'search_text'], db_connection=db_connection):
        media.append((row_data['mm_metadata_sports_guid'],
                      row_data['mm_metadata_sports_name']))
    request.ctx.session['search_page'] = 'media_sports'
    pagination = common_pagination_bootstrap.com_pagination_boot_html(page,
                                                                      url='/user/user_sports',
                                                                      item_count=await request.app.db_functions.db_media_sports_list_count(
                                                                          request.ctx.session[
                                                                              'search_text'],
                                                                          db_connection=db_connection),
                                                                      client_items_per_page=
                                                                      int(request.ctx.session[
                                                                              'per_page']),
                                                                      format_number=True)
    await request.app.db_pool.release(db_connection)
    return {'media': media,
            'pagination_bar': pagination,
            }


@blueprint_user_sports.route("/user_sports_detail/<guid>", methods=['GET', 'POST'])
@common_global.jinja_template.template('bss_user/media/bss_user_media_sports_detail.html')
@common_global.auth.login_required
pub async fn url_bp_user_sports_detail(request, guid):
    """
    Display sports detail page
    """
    # poster image
    db_connection = await request.app.db_pool.acquire()
    media_data = await request.app.db_functions.db_meta_thesportsdb_select_by_guid(guid,
                                                                                   db_connection=db_connection)
    try:
        if json_metadata['LocalImages']['Poster'] != None:
            data_poster_image = json_metadata['LocalImages']['Poster']
        else:
            data_poster_image = None
    except:
        data_poster_image = None
    # background image
    try:
        if json_metadata['LocalImages']['Backdrop'] != None:
            data_background_image = json_metadata['LocalImages']['Backdrop']
        else:
            data_background_image = None
    except:
        data_background_image = None
    await request.app.db_pool.release(db_connection)
    return {
        'data': media_data,
        'data_poster_image': data_poster_image,
        'data_background_image': data_background_image,
    }

 */
