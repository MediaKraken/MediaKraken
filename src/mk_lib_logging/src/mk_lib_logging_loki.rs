use chrono::Utc;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::sync::OnceLock;
use tokio::time::Duration;

const LOKI_PUSH_URL: &str = "http://loki.monitoring.svc.mkcluster.local:3100/loki/api/v1/push";
const LOKI_QUERY_URL: &str =
    "http://loki-gateway.monitoring.svc.mkcluster.local/loki/api/v1/query_range";
const LOKI_MAX_ENTRY_BYTES: usize = 262_144;
const LOKI_SAFE_ENTRY_BYTES: usize = 240_000;

const MAX_RETRIES: u32 = 5;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const READ_WINDOW_NS: i64 = 24 * 60 * 60 * 1_000_000_000;
const READ_LIMIT: &str = "100";

type BoxError = Box<dyn std::error::Error + Send + Sync>;

// Loki responses come in two shapes:
//   {"status":"success","data":{...}}
//   {"status":"error","error":"..."}
// `data` must stay optional so error responses deserialize.
#[derive(Debug, Deserialize)]
struct LokiResponse {
    status: String,
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    data: Option<LokiData>,
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

#[derive(Debug)]
pub struct LokiLog {
    pub timestamp_ns: i128,
    pub labels: String,
    pub line: String,
}

fn retrying_client() -> &'static ClientWithMiddleware {
    static CLIENT: OnceLock<ClientWithMiddleware> = OnceLock::new();
    CLIENT.get_or_init(|| {
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(MAX_RETRIES);
        ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build()
    })
}

fn truncate_to_bytes(input: &str, max_bytes: usize) -> String {
    if input.len() <= max_bytes {
        return input.to_string();
    }
    let mut end = max_bytes;
    while !input.is_char_boundary(end) {
        end -= 1;
    }
    input[..end].to_string()
}

pub async fn mk_logging_loki_push(message_text: serde_json::Value) -> Result<(), BoxError> {
    let exe_path = env::current_exe()?;
    let exe_name = exe_path
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .ok_or("Invalid executable name")?;

    let original_log_line = serde_json::to_string(&message_text)?;
    let original_size = original_log_line.len();

    let log_line = if original_size > LOKI_SAFE_ENTRY_BYTES {
        let truncated = truncate_to_bytes(&original_log_line, LOKI_SAFE_ENTRY_BYTES);
        json!({
            "truncated": true,
            "original_size_bytes": original_size,
            "max_entry_bytes": LOKI_MAX_ENTRY_BYTES,
            "message_preview": truncated
        })
        .to_string()
    } else {
        original_log_line
    };

    let timestamp_ns = Utc::now()
        .timestamp_nanos_opt()
        .ok_or("failed to generate nanosecond timestamp")?
        .to_string();

    let payload = json!({
        "streams": [
            {
                "stream": {
                    "job": "mediakraken",
                    "app": exe_name,
                    "format": "json"
                },
                "values": [
                    [timestamp_ns, log_line]
                ]
            }
        ]
    });

    let resp = retrying_client()
        .post(LOKI_PUSH_URL)
        .timeout(REQUEST_TIMEOUT)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();

    if !status.is_success() {
        return Err(format!("loki push failed: status={} body={}", status, body).into());
    }

    Ok(())
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

fn escape_logql_string(value: &str) -> String {
    value
        .replace('\\', r#"\\"#)
        .replace('"', r#"\""#)
        .replace('\n', r#"\n"#)
        .replace('\r', r#"\r"#)
}

pub async fn mk_logging_loki_read(message_type: &str) -> Result<Vec<LokiLog>, BoxError> {
    let now_ns = Utc::now()
        .timestamp_nanos_opt()
        .ok_or("failed to generate nanosecond timestamp")?;
    let start_ns = now_ns - READ_WINDOW_NS;

    // `message_type` names an app/job — scope via the `app` label, not a line
    // substring filter, so Loki can use its index.
    let query = if message_type.is_empty() {
        r#"{job="mediakraken"}"#.to_string()
    } else {
        format!(
            r#"{{job="mediakraken",app="{}"}}"#,
            escape_logql_string(message_type)
        )
    };

    let resp: LokiResponse = retrying_client()
        .get(LOKI_QUERY_URL)
        .timeout(REQUEST_TIMEOUT)
        .query(&[
            ("query", query.as_str()),
            ("limit", READ_LIMIT),
            ("direction", "backward"),
            ("start", &start_ns.to_string()),
            ("end", &now_ns.to_string()),
        ])
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    if resp.status != "success" {
        let loki_err = resp
            .error
            .unwrap_or_else(|| "unknown loki error".to_string());
        return Err(format!("loki query failed: {}", loki_err).into());
    }

    let data = resp
        .data
        .ok_or("loki returned success but data field was missing")?;

    let capacity = data
        .result
        .iter()
        .map(|stream| stream.values.len())
        .sum::<usize>();

    let mut logs = Vec::with_capacity(capacity);

    for stream in data.result {
        let stream_str = stream_to_logql(&stream.stream);
        for [ts, line] in stream.values {
            let Ok(timestamp_ns) = ts.parse() else {
                continue;
            };
            logs.push(LokiLog {
                timestamp_ns,
                labels: stream_str.clone(),
                line,
            });
        }
    }

    Ok(logs)
}
