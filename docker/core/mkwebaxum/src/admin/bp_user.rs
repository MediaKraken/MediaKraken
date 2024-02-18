use askama::Template;
use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension,
};
use axum_session_auth::{Auth, AuthSession, Rights, SessionPgPool};
use mk_lib_common::mk_lib_common_pagination;
use mk_lib_database;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_user.html")]
struct TemplateAdminUserContext<'a> {
    template_data: &'a Vec<mk_lib_database::mk_lib_database_user::DBUserList>,
    pagination_bar: &'a String,
    page: &'a usize,
}

pub async fn admin_user(
    Extension(sqlx_pool): Extension<PgPool>,
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
        let reply_html = template.render().unwrap();
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let db_offset: i64 = (page * 30) - 30;
        let total_pages: i64 = mk_lib_database::mk_lib_database_user::mk_lib_database_user_count(
            &sqlx_pool,
            String::new(),
        )
        .await
        .unwrap();
        let pagination_html = mk_lib_common_pagination::mk_lib_common_paginate(
            total_pages,
            page,
            "/admin/user".to_string(),
        )
        .await
        .unwrap();
        let user_list = mk_lib_database::mk_lib_database_user::mk_lib_database_user_read(
            &sqlx_pool, db_offset, 30,
        )
        .await
        .unwrap();
        let page_usize = page as usize;
        let template = TemplateAdminUserContext {
            template_data: &user_list,
            pagination_bar: &pagination_html,
            page: &page_usize,
        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_user_detail.html")]
struct TemplateAdminUserDetailContext {}

pub async fn admin_user_detail(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Path(guid): Path<uuid::Uuid>,
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
        let reply_html = template.render().unwrap();
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let template = TemplateAdminUserDetailContext {};
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

// #[derive(Template)]
// #[template(path = "bss_admin/bss_admin_user_delete.html")]
// struct TemplateAdminUserDeleteContext {}

// pub async fn admin_user_delete(
//     Extension(sqlx_pool): Extension<PgPool>,
//     Path(guid): Path<uuid::Uuid>,
//     method: Method,
//     auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
// ) -> impl IntoResponse {
//     let current_user = auth.current_user.clone().unwrap_or_default();
//     if !Auth::<mk_lib_database::mk_lib_database_user::User, i64, PgPool>::build(
//         [Method::GET],
//         false,
//     )
//     .requires(Rights::any([Rights::permission("Admin::View")]))
//     .validate(&current_user, &method, None)
//     .await
//     {
//         let template = TemplateError403Context {};
//         let reply_html = template.render().unwrap();
//         (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
//     } else {
//         let template = TemplateAdminUserDeleteContext {};
//         let reply_html = template.render().unwrap();
//         (StatusCode::OK, Html(reply_html).into_response())
//     }
// }
