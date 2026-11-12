use crate::axum_custom_filters::filters;
use crate::mk_lib_logging;
use askama::Template;
use axum::{
    extract::Query,
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use serde::Deserialize;
use sqlx::postgres::PgPool;
use std::time::Duration;

/// Loki gateway URL for the `app` label-values API
/// (`GET /loki/api/v1/label/app/values`). Same gateway host as the
/// `query_range` endpoint used by `mk_lib_logging`.
const LOKI_APP_LABEL_VALUES_URL: &str =
    "http://loki-gateway.monitoring.svc.mkcluster.local/loki/api/v1/label/app/values";

/// Lookback window (seconds) shared with `mk_logging_loki_read` so the app
/// dropdown lists exactly the apps that can appear in the log table.
const LOKI_APP_LIST_WINDOW_SECS: i64 = 24 * 60 * 60;

#[derive(Debug, Deserialize)]
struct LokiLabelValuesResponse {
    status: String,
    #[serde(default)]
    data: Option<Vec<String>>,
}

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {
    is_admin: bool,
}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_logging.html")]
struct TemplateLogContext<'a> {
    template_data: &'a Vec<mk_lib_logging::mk_lib_logging_loki::LokiLog>,
    template_data_exists: &'a bool,
    template_apps: &'a Vec<String>,
    template_selected_app: &'a String,
    page_title: Option<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct LoggingQueryParams {
    pub app: Option<String>,
}

impl TemplateLogContext<'_> {
    /// Human-readable UTC rendering of a Loki nanosecond timestamp with
    /// milliseconds (e.g. "2026-11-09 12:34:56.123 UTC"). Values that are
    /// not a valid Unix timestamp fall back to the raw number.
    fn timestamp_to_datetime(&self, timestamp_ns: &i128) -> String {
        let Ok(nanos) = i64::try_from(*timestamp_ns) else {
            return timestamp_ns.to_string();
        };
        // `rem_euclid` is in [0, 1e9), which always fits a u32.
        let secs = nanos.div_euclid(1_000_000_000);
        let nsecs = nanos.rem_euclid(1_000_000_000) as u32;
        chrono::DateTime::<chrono::Utc>::from_timestamp(secs, nsecs)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S.%3f UTC").to_string())
            .unwrap_or_else(|| timestamp_ns.to_string())
    }
}

/// App names are pushed to Loki as executable names, so restrict the filter
/// to a safe label-value character set; anything else is treated as "no filter".
fn sanitize_app_name(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed.len() > 255 {
        return None;
    }
    if !trimmed
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '.' || c == '_')
    {
        return None;
    }
    Some(trimmed.to_string())
}

/// Distinct `app` label values from Loki over the same 24-hour window as the
/// log read. Failures are non-fatal: the page renders without app filter
/// options.
async fn fetch_app_names() -> Vec<String> {
    let Some(now_ns) = chrono::Utc::now().timestamp_nanos_opt() else {
        return Vec::new();
    };
    let start_ns = now_ns - (LOKI_APP_LIST_WINDOW_SECS * 1_000_000_000);

    let url = match reqwest::Url::parse_with_params(
        LOKI_APP_LABEL_VALUES_URL,
        [
            ("start", start_ns.to_string().as_str()),
            ("end", now_ns.to_string().as_str()),
        ],
    ) {
        Ok(url) => url,
        Err(_) => return Vec::new(),
    };

    let response = match reqwest::Client::new()
        .get(url)
        .timeout(Duration::from_secs(30))
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => return Vec::new(),
    };
    let response = match response.error_for_status() {
        Ok(response) => response,
        Err(_) => return Vec::new(),
    };
    let parsed: LokiLabelValuesResponse = match response.json().await {
        Ok(parsed) => parsed,
        Err(_) => return Vec::new(),
    };
    if parsed.status != "success" {
        return Vec::new();
    }

    let mut apps = parsed.data.unwrap_or_default();
    apps.sort();
    apps.dedup();
    apps
}

pub async fn admin_logging(
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Query(params): Query<LoggingQueryParams>,
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
        let reply_html = template.render().unwrap_or_default();
        (StatusCode::FORBIDDEN, Html(reply_html).into_response())
    } else {
        let selected_app = params
            .app
            .as_deref()
            .and_then(sanitize_app_name)
            .unwrap_or_default();
        // The loki read's error type (`Box<dyn Error>`) is not `Send`, so resolve
        // it inside the async block: `tokio::join!` would otherwise keep the
        // non-`Send` value in its state and the handler future would not be `Send`.
        let (logging_list, apps) = tokio::join!(
            async {
                mk_lib_logging::mk_lib_logging_loki::mk_logging_loki_read(&selected_app)
                    .await
                    .unwrap_or_default()
            },
            fetch_app_names(),
        );
        let logging_data = !logging_list.is_empty();
        let template = TemplateLogContext {
            template_data: &logging_list,
            template_data_exists: &logging_data,
            template_apps: &apps,
            template_selected_app: &selected_app,
            page_title: Some("MediaKraken Admin Logging".to_string()),
        };
        let reply_html = template.render().unwrap_or_default();
        (StatusCode::OK, Html(reply_html).into_response())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_app_name_accepts_executable_names() {
        assert_eq!(
            sanitize_app_name("mkwebaxum"),
            Some("mkwebaxum".to_string())
        );
        assert_eq!(
            sanitize_app_name("mk-transcode_v2.0"),
            Some("mk-transcode_v2.0".to_string())
        );
        assert_eq!(sanitize_app_name("  mkcron  "), Some("mkcron".to_string()));
    }

    #[test]
    fn sanitize_app_name_rejects_unsafe_values() {
        assert_eq!(sanitize_app_name(""), None);
        assert_eq!(sanitize_app_name("   "), None);
        assert_eq!(sanitize_app_name("mk; drop"), None);
        assert_eq!(sanitize_app_name(r#"mk"webaxum"#), None);
        assert_eq!(sanitize_app_name("a/b"), None);
        assert_eq!(sanitize_app_name(&"a".repeat(256)), None);
    }

    #[test]
    fn logging_template_renders_app_filter() {
        let logs = vec![mk_lib_logging::mk_lib_logging_loki::LokiLog {
            timestamp_ns: 1_762_800_000_000_000_000,
            labels: r#"app="mkwebaxum",format="json",job="mediakraken""#.to_string(),
            line: "hello".to_string(),
        }];
        let exists = true;
        let apps = vec!["mkwebaxum".to_string(), "mktranscode".to_string()];
        let selected = "mkwebaxum".to_string();
        let template = TemplateLogContext {
            template_data: &logs,
            template_data_exists: &exists,
            template_apps: &apps,
            template_selected_app: &selected,
            page_title: Some("MediaKraken Admin Logging".to_string()),
        };
        let html = template.render().unwrap();
        assert!(html.contains(r#"href="/admin/logging""#));
        assert!(html.contains(r#"href="/admin/logging?app=mkwebaxum""#));
        assert!(html.contains(r#"href="/admin/logging?app=mktranscode""#));
        assert!(html.contains("All apps"));
        assert!(html.contains("for app"));
        assert!(html.contains("mkwebaxum"));
    }
}
