use crate::AppState;
use crate::mk_lib_database;
use crate::user_preferences;
use askama::Template;
use axum::extract::State;
use axum::{
    Extension,
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session::{SessionConfig, SessionLayer};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use mk_lib_common::mk_lib_common_pagination;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_report_known_media.html")]
struct TemplateReportKnownMediaContext<'a> {
    template_data: &'a Vec<mk_lib_database::mk_lib_database_report::DBReportKnownMediaList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
    page_title: Option<String>,
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
        let template = TemplateError403Context {};
        let reply_html = template.render().map_err(|e| e.to_string())?;
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
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
            ?;
        let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
            total_pages,
            page,
            "/admin/report_known_media".to_string(),
            None,
            pagination_count,
        )
        .await
        ?;
        let report_list =
            mk_lib_database::mk_lib_database_report::mk_lib_database_report_known_media_read(
                &state.sqlx_pool_ro,
                db_offset,
                pagination_count,
            )
            .await
            ?;
        let mut report_data: bool = false;
        if report_list.len() > 0 {
            report_data = true;
        }
        let page_usize = page as usize;
        let template = TemplateReportKnownMediaContext {
            template_data: &report_list,
            template_data_exists: &report_data,
            pagination_bar: &pagination_html,
            page: &page_usize,
            page_title: Some("MediaKraken Admin Report".to_string()),
        };
        let reply_html = template.render().map_err(|e| e.to_string())?;
        (StatusCode::OK, Html(reply_html).into_response())
    }
}
