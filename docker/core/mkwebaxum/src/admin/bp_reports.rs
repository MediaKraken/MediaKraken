use std::borrow::Cow;

use crate::AppState;
use crate::admin::bp_library::media_class_description;
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
use mk_lib_common::mk_lib_common_pagination;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {
    is_admin: bool,
}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_report_known_media.html")]
struct TemplateReportKnownMediaContext<'a> {
    template_data: &'a Vec<mk_lib_database::mk_lib_database_report::DBReportKnownMediaList>,
    #[allow(dead_code)]
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
    page_title: Option<String>,
}

impl TemplateReportKnownMediaContext<'_> {
    /// Display name for a media class enum value (see the shared
    /// `media_class_description` free function).
    fn media_class_description(&self, value: &i16) -> Cow<'static, str> {
        media_class_description(*value)
    }
}

pub async fn admin_report_known_media(
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
    .requires(Rights::any([Rights::permission("Admin::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        let template = TemplateError403Context { is_admin: true };
        let reply_html = template.render().unwrap();
        (StatusCode::FORBIDDEN, Html(reply_html).into_response())
    } else {
        let pagination_count =
            user_preferences::load_user_pagination_count(&state.sqlx_pool_ro, current_user.id)
                .await
                .unwrap_or(user_preferences::DEFAULT_PAGINATION_COUNT);
        let db_offset: i64 = (page * pagination_count) - pagination_count;
        let total_pages: i64 =
            mk_lib_database::mk_lib_database_report::mk_lib_database_report_known_media_count(
                &state.sqlx_pool_ro,
            )
            .await
            .unwrap();
        let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
            total_pages,
            page,
            "/admin/report_known_media".to_string(),
            None,
            pagination_count,
        )
        .await
        .unwrap();
        let report_list =
            mk_lib_database::mk_lib_database_report::mk_lib_database_report_known_media_read(
                &state.sqlx_pool_ro,
                db_offset,
                pagination_count,
            )
            .await
            .unwrap();
        let mut report_data: bool = false;
        if !report_list.is_empty() {
            report_data = true;
        }
        let page_usize = page as usize;
        let template = TemplateReportKnownMediaContext {
            template_data: &report_list,
            template_data_exists: &report_data,
            pagination_bar: &pagination_html,
            page: &page_usize,
            page_title: Some("MediaKraken Admin Report - Known Media".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

/// One row of the unmatched media report: the file path plus the fields
/// `parse_torrent_title` extracted from it, so an admin can see what the
/// parser identifies for each file.
struct UnmatchedMediaParsedRow {
    mm_media_path: String,
    parse_title: String,
    parse_season: String,
    parse_episode: String,
    parse_resolution: String,
    parse_group: String,
}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_report_unmatched_media.html")]
struct TemplateReportUnmatchedMediaContext<'a> {
    template_data: &'a Vec<UnmatchedMediaParsedRow>,
    #[allow(dead_code)]
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
    page_title: Option<String>,
}

pub async fn admin_report_unmatched_media(
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
    .requires(Rights::any([Rights::permission("Admin::View")]))
    .validate(&current_user, &method, None)
    .await
    {
        let template = TemplateError403Context { is_admin: true };
        let reply_html = template.render().unwrap();
        (StatusCode::FORBIDDEN, Html(reply_html).into_response())
    } else {
        let pagination_count =
            user_preferences::load_user_pagination_count(&state.sqlx_pool_ro, current_user.id)
                .await
                .unwrap_or(user_preferences::DEFAULT_PAGINATION_COUNT);
        let db_offset: i64 = (page * pagination_count) - pagination_count;
        let total_pages: i64 =
            mk_lib_database::database_media::mk_lib_database_media::mk_lib_database_media_unmatched_count(
                &state.sqlx_pool_ro,
            )
            .await
            .unwrap();
        let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
            total_pages,
            page,
            "/admin/report_unmatched_media".to_string(),
            None,
            pagination_count,
        )
        .await
        .unwrap();
        let report_list =
            mk_lib_database::database_media::mk_lib_database_media::mk_lib_database_media_unmatched_read(
                &state.sqlx_pool_ro,
                db_offset,
                pagination_count,
            )
            .await
            .unwrap();
        let parsed_report: Vec<UnmatchedMediaParsedRow> = report_list
            .into_iter()
            .map(|entry| {
                let parsed = parse_torrent_title::parse(&entry.mm_media_path);
                UnmatchedMediaParsedRow {
                    mm_media_path: entry.mm_media_path,
                    parse_title: parsed.title().to_string(),
                    parse_season: parsed
                        .season()
                        .map_or(String::new(), |value| value.to_string()),
                    parse_episode: parsed
                        .episode()
                        .map_or(String::new(), |value| value.to_string()),
                    parse_resolution: parsed.resolution().unwrap_or_default().to_string(),
                    parse_group: parsed.group().unwrap_or_default().to_string(),
                }
            })
            .collect();
        let mut report_data: bool = false;
        if !parsed_report.is_empty() {
            report_data = true;
        }
        let page_usize = page as usize;
        let template = TemplateReportUnmatchedMediaContext {
            template_data: &parsed_report,
            template_data_exists: &report_data,
            pagination_bar: &pagination_html,
            page: &page_usize,
            page_title: Some("MediaKraken Admin Report - Unmatched Media".to_string()),
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}
