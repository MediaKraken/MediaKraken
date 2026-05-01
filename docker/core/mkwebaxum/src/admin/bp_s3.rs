use crate::axum_custom_filters::filters;
use crate::mk_lib_database;
use askama::Template;
use aws_config::BehaviorVersion;
use aws_sdk_s3::{Client as S3Client, config::Builder as S3ConfigBuilder};
use axum::extract::Query;
use axum::{
    http::{Method, StatusCode},
    response::{Html, IntoResponse},
};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use serde::Deserialize;
use sqlx::postgres::PgPool;
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Template)]
#[template(path = "bss_error/bss_error_403.html")]
struct TemplateError403Context {}

#[derive(Clone)]
pub struct BucketSummary {
    pub name: String,
    pub object_count: u64,
    pub total_size: u64,
}

const SUMMARY_CACHE_TTL: Duration = Duration::from_secs(60);

#[derive(Clone)]
struct CachedSummaries {
    captured_at: Instant,
    summaries: Vec<BucketSummary>,
}

fn summary_cache() -> &'static RwLock<Option<CachedSummaries>> {
    static CACHE: OnceLock<RwLock<Option<CachedSummaries>>> = OnceLock::new();
    CACHE.get_or_init(|| RwLock::new(None))
}

async fn cached_bucket_summaries(client: &S3Client) -> Result<Vec<BucketSummary>, String> {
    {
        let guard = summary_cache().read().await;
        if let Some(cached) = guard.as_ref() {
            if cached.captured_at.elapsed() < SUMMARY_CACHE_TTL {
                return Ok(cached.summaries.clone());
            }
        }
    }
    let fresh = gather_bucket_summaries(client).await?;
    let mut guard = summary_cache().write().await;
    *guard = Some(CachedSummaries {
        captured_at: Instant::now(),
        summaries: fresh.clone(),
    });
    Ok(fresh)
}

pub struct BrowserFolder {
    pub name: String,
    pub prefix: String,
}

pub struct BrowserObject {
    pub name: String,
    pub key: String,
    pub size: u64,
    pub last_modified: String,
}

pub struct Breadcrumb {
    pub label: String,
    pub prefix: String,
}

#[derive(Template)]
#[template(path = "bss_admin/bss_admin_s3.html")]
struct AdminS3Template<'a> {
    template_endpoint: &'a String,
    template_region: &'a String,
    template_error: &'a Option<String>,
    template_bucket_count: &'a u64,
    template_object_count: &'a u64,
    template_total_size: &'a u64,
    template_data_path: &'a Option<String>,
    template_disk_total: &'a Option<u64>,
    template_disk_free: &'a Option<u64>,
    template_buckets: &'a Vec<BucketSummary>,
    template_browse_active: &'a bool,
    template_selected_bucket: &'a String,
    template_breadcrumbs: &'a Vec<Breadcrumb>,
    template_folders: &'a Vec<BrowserFolder>,
    template_objects: &'a Vec<BrowserObject>,
    template_truncated: &'a bool,
    page_title: Option<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct S3QueryParams {
    pub bucket: Option<String>,
    pub prefix: Option<String>,
}

fn build_s3_client_config() -> Option<(String, String)> {
    let endpoint = "garage-api.garage".to_string();
    let region = "garage".to_string();
    Some((endpoint, region))
}

async fn build_s3_client(endpoint: &str, region: &str) -> S3Client {
    let shared = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let s3_config = S3ConfigBuilder::from(&shared)
        .endpoint_url(endpoint.to_string())
        .region(aws_sdk_s3::config::Region::new(region.to_string()))
        .force_path_style(true)
        .build();
    S3Client::from_conf(s3_config)
}

async fn gather_bucket_summaries(client: &S3Client) -> Result<Vec<BucketSummary>, String> {
    let buckets_response = client
        .list_buckets()
        .send()
        .await
        .map_err(|err| format!("list_buckets failed: {err}"))?;
    let bucket_list = buckets_response.buckets();
    let mut summaries: Vec<BucketSummary> = Vec::with_capacity(bucket_list.len());
    for bucket in bucket_list {
        let Some(name) = bucket.name() else {
            continue;
        };
        let mut object_count: u64 = 0;
        let mut total_size: u64 = 0;
        let mut continuation_token: Option<String> = None;
        loop {
            let mut req = client.list_objects_v2().bucket(name);
            if let Some(token) = continuation_token.as_deref() {
                req = req.continuation_token(token);
            }
            let page = match req.send().await {
                Ok(page) => page,
                Err(err) => {
                    tracing::warn!(bucket = %name, error = %err, "list_objects_v2 failed");
                    break;
                }
            };
            for object in page.contents() {
                object_count += 1;
                if let Some(size) = object.size() {
                    total_size = total_size.saturating_add(u64::try_from(size).unwrap_or(0));
                }
            }
            if page.is_truncated().unwrap_or(false) {
                continuation_token = page.next_continuation_token().map(|s| s.to_string());
                if continuation_token.is_none() {
                    break;
                }
            } else {
                break;
            }
        }
        summaries.push(BucketSummary {
            name: name.to_string(),
            object_count,
            total_size,
        });
    }
    summaries.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(summaries)
}

async fn browse_bucket(
    client: &S3Client,
    bucket: &str,
    prefix: &str,
) -> Result<(Vec<BrowserFolder>, Vec<BrowserObject>, bool), String> {
    let mut request = client
        .list_objects_v2()
        .bucket(bucket)
        .delimiter("/")
        .max_keys(1000);
    if !prefix.is_empty() {
        request = request.prefix(prefix);
    }
    let response = request
        .send()
        .await
        .map_err(|err| format!("list_objects_v2 failed: {err}"))?;

    let mut folders: Vec<BrowserFolder> = Vec::new();
    for cp in response.common_prefixes() {
        if let Some(folder_prefix) = cp.prefix() {
            let trimmed = folder_prefix.strip_suffix('/').unwrap_or(folder_prefix);
            let display = trimmed.rsplit('/').next().unwrap_or(trimmed).to_string();
            folders.push(BrowserFolder {
                name: display,
                prefix: folder_prefix.to_string(),
            });
        }
    }

    let mut objects: Vec<BrowserObject> = Vec::new();
    for obj in response.contents() {
        let Some(key) = obj.key() else {
            continue;
        };
        if key == prefix {
            continue;
        }
        let display = key
            .strip_prefix(prefix)
            .unwrap_or(key)
            .trim_start_matches('/')
            .to_string();
        if display.is_empty() {
            continue;
        }
        let size = obj.size().and_then(|s| u64::try_from(s).ok()).unwrap_or(0);
        let last_modified = obj
            .last_modified()
            .and_then(|ts| chrono::DateTime::<chrono::Utc>::from_timestamp(ts.secs(), 0))
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S UTC").to_string())
            .unwrap_or_default();
        objects.push(BrowserObject {
            name: display,
            key: key.to_string(),
            size,
            last_modified,
        });
    }

    let truncated = response.is_truncated().unwrap_or(false);
    Ok((folders, objects, truncated))
}

fn build_breadcrumbs(prefix: &str) -> Vec<Breadcrumb> {
    let mut crumbs = Vec::new();
    if prefix.is_empty() {
        return crumbs;
    }
    let mut accumulator = String::new();
    for segment in prefix.trim_end_matches('/').split('/') {
        if segment.is_empty() {
            continue;
        }
        accumulator.push_str(segment);
        accumulator.push('/');
        crumbs.push(Breadcrumb {
            label: segment.to_string(),
            prefix: accumulator.clone(),
        });
    }
    crumbs
}

fn read_disk_stats(path: &str) -> Option<(u64, u64)> {
    let cpath = std::ffi::CString::new(path).ok()?;
    let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
    let ret = unsafe { libc::statvfs(cpath.as_ptr(), &mut stat) };
    if ret != 0 {
        return None;
    }
    let frsize = u64::try_from(stat.f_frsize).unwrap_or(0);
    let blocks = u64::try_from(stat.f_blocks).unwrap_or(0);
    let avail = u64::try_from(stat.f_bavail).unwrap_or(0);
    Some((blocks.saturating_mul(frsize), avail.saturating_mul(frsize)))
}

fn sanitize_bucket_name(name: &str) -> Option<String> {
    let trimmed = name.trim();
    if trimmed.is_empty() || trimmed.len() > 63 {
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

fn sanitize_prefix(prefix: &str) -> String {
    let mut cleaned = prefix.trim_start_matches('/').to_string();
    if cleaned.len() > 1024 {
        cleaned.truncate(1024);
    }
    cleaned
}

pub async fn admin_s3(
    method: Method,
    auth: AuthSession<mk_lib_database::mk_lib_database_user::User, i64, SessionPgPool, PgPool>,
    Query(params): Query<S3QueryParams>,
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
        return (StatusCode::UNAUTHORIZED, Html(reply_html).into_response());
    }

    let selected_bucket = params.bucket.as_deref().and_then(sanitize_bucket_name);
    let current_prefix = sanitize_prefix(params.prefix.as_deref().unwrap_or(""));

    let mut error_message: Option<String> = None;
    let mut bucket_summaries: Vec<BucketSummary> = Vec::new();
    let mut folders: Vec<BrowserFolder> = Vec::new();
    let mut objects: Vec<BrowserObject> = Vec::new();
    let mut truncated = false;

    let (endpoint, region) = match build_s3_client_config() {
        Some(values) => values,
        None => {
            error_message = Some(
                "S3 endpoint is not configured (set MK_GARAGE_S3_ENDPOINT or AWS_ENDPOINT_URL)."
                    .to_string(),
            );
            (String::new(), String::new())
        }
    };

    if error_message.is_none() {
        let client = build_s3_client(&endpoint, &region).await;
        match cached_bucket_summaries(&client).await {
            Ok(summaries) => bucket_summaries = summaries,
            Err(err) => {
                tracing::warn!(error = %err, "failed to gather S3 bucket summaries");
                error_message = Some(format!("Unable to query S3 backend: {err}"));
            }
        }
        if let Some(bucket_name) = selected_bucket.as_deref() {
            match browse_bucket(&client, bucket_name, &current_prefix).await {
                Ok((listed_folders, listed_objects, is_truncated)) => {
                    folders = listed_folders;
                    objects = listed_objects;
                    truncated = is_truncated;
                }
                Err(err) => {
                    tracing::warn!(bucket = %bucket_name, error = %err, "failed to list bucket");
                    error_message = Some(match error_message.take() {
                        Some(existing) => format!("{existing}; bucket browse failed: {err}"),
                        None => format!("Unable to browse bucket: {err}"),
                    });
                }
            }
        }
    }

    let bucket_count: u64 = u64::try_from(bucket_summaries.len()).unwrap_or(0);
    let object_count: u64 = bucket_summaries.iter().map(|b| b.object_count).sum();
    let total_size: u64 = bucket_summaries.iter().map(|b| b.total_size).sum();

    let data_path = std::env::var("MK_GARAGE_DATA_PATH")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let (disk_total, disk_free) = match data_path.as_deref() {
        Some(path) => match read_disk_stats(path) {
            Some((total, free)) => (Some(total), Some(free)),
            None => (None, None),
        },
        None => (None, None),
    };

    let breadcrumbs = build_breadcrumbs(&current_prefix);
    let browse_active = selected_bucket.is_some();
    let selected_bucket_string = selected_bucket.clone().unwrap_or_default();

    let template = AdminS3Template {
        template_endpoint: &endpoint,
        template_region: &region,
        template_error: &error_message,
        template_bucket_count: &bucket_count,
        template_object_count: &object_count,
        template_total_size: &total_size,
        template_data_path: &data_path,
        template_disk_total: &disk_total,
        template_disk_free: &disk_free,
        template_buckets: &bucket_summaries,
        template_browse_active: &browse_active,
        template_selected_bucket: &selected_bucket_string,
        template_breadcrumbs: &breadcrumbs,
        template_folders: &folders,
        template_objects: &objects,
        template_truncated: &truncated,
        page_title: Some("MediaKraken Admin S3".to_string()),
    };
    let reply_html = template.render().unwrap_or_default();
    (StatusCode::OK, Html(reply_html).into_response())
}
