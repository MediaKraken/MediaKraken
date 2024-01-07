use crate::guard;
use askama::Template;
use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse,},
    Extension,
};
use axum_session_auth::{AuthSession, SessionPgPool};
use mk_lib_database;
use sqlx::postgres::PgPool;
use mk_lib_common::mk_lib_common_pagination;

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_report_known_media.html")]
struct TemplateReportKnownMediaContext<'a> {
    template_data: &'a Vec<mk_lib_database::mk_lib_database_report::DBReportKnownMediaList>,
    template_data_exists: &'a bool,
    pagination_bar: &'a String,
    page: &'a usize,
}

pub async fn admin_report_known_media(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(page): Path<i64>,
) -> impl IntoResponse {
    let auth_response = guard::guard_page_by_user(method, auth, true);
    let db_offset: i64 = (page * 30) - 30;
    let total_pages: i64 = mk_lib_database::mk_lib_database_report::mk_lib_database_report_known_media_count(
        &sqlx_pool,
    )
    .await
    .unwrap();
    let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
        total_pages,
        page,
        "/admin/report_known_media".to_string(),
    )
    .await
    .unwrap();
    let report_list =
        mk_lib_database::mk_lib_database_report::mk_lib_database_report_known_media_read(&sqlx_pool, db_offset, 30)
            .await
            .unwrap();
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
    };
    let reply_html = template.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}
