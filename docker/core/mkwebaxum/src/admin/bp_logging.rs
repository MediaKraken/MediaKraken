use crate::mk_lib_logging;
use askama::Template;
use axum::{
    extract::Path,
    http::{Method, StatusCode},
    response::{Html, IntoResponse, Redirect},
    Extension,
};
use axum_session::{SessionConfig, SessionLayer};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use sqlx::postgres::PgPool;

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_logging.html")]
struct TemplateLogContext<'a> {
    template_data: &'a Vec<mk_lib_logging::mk_lib_logging_loki::LokiLog>,
    template_data_exists: &'a bool,
        page_title: Option<String>,

}

pub async fn admin_logging(
    Extension(sqlx_pool): Extension<PgPool>,
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
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
        let logging_list =
            mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_read("fake_query")
                .await
                .unwrap();
        let mut logging_data: bool = false;
        if logging_list.len() > 0 {
            logging_data = true;
        }
        let template = TemplateLogContext {
            template_data: &logging_list,
            template_data_exists: &logging_data,
                        page_title: Some("MediaKraken Admin Logging".to_string()),

        };
        let reply_html = template.render().unwrap();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}
