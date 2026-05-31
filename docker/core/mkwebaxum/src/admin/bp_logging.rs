use crate::mk_lib_logging;
use askama::Template;
use axum::{
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use reqwest::Client;
use serde::Deserialize;
use sqlx::postgres::PgPool;
use std::collections::HashMap;
use std::env;

#[derive(Debug, Deserialize)]
struct LokiResponse {
    data: LokiData,
}

#[derive(Debug, Deserialize)]
struct LokiData {
    result: Vec<LokiStream>,
}

#[derive(Debug, Deserialize)]
struct LokiStream {
    stream: HashMap<String, String>,
    values: Vec<[String; 2]>,
}

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

fn stream_labels_to_string(labels: &HashMap<String, String>) -> String {
    let mut pairs: Vec<_> = labels.iter().collect();
    pairs.sort_by_key(|(k, _)| *k);
    pairs
        .into_iter()
        .map(|(k, v)| format!(r#"{k}="{v}""#))
        .collect::<Vec<_>>()
        .join(",")
}

fn stream_to_logql(labels: &HashMap<String, String>) -> String {
    format!("{{{}}}", stream_labels_to_string(labels))
}

pub async fn admin_logging(
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
        let reply_html = template.render().unwrap_or_default();
        (StatusCode::UNAUTHORIZED, Html(reply_html).into_response())
    } else {
        let logging_list = mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_read("")
            .await
            .unwrap_or_default();
        let logging_data = !logging_list.is_empty();
        let template = TemplateLogContext {
            template_data: &logging_list,
            template_data_exists: &logging_data,
            page_title: Some("MediaKraken Admin Logging".to_string()),
        };
        let reply_html = template.render().unwrap_or_default();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}
