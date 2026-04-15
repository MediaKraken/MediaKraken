use chrono::Utc;
use reqwest::Client;
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

#[derive(Debug)]
pub struct LokiLog {
    pub timestamp_ns: i128,
    pub labels: String,
    pub line: String,
}

fn retrying_client() -> &'static ClientWithMiddleware {
    static CLIENT: OnceLock<ClientWithMiddleware> = OnceLock::new();
    CLIENT.get_or_init(|| {
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(100);
        ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build()
    })
}

fn query_client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();
    CLIENT.get_or_init(Client::new)
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

pub async fn mk_logging_loki_push(
    message_text: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
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
        .timeout(Duration::from_secs(30))
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
    value.replace('\\', r#"\\"#).replace('"', r#"\""#)
}

pub async fn mk_logging_loki_read(
    message_type: &str,
) -> Result<Vec<LokiLog>, Box<dyn std::error::Error>> {
    let now_ns = Utc::now()
        .timestamp_nanos_opt()
        .ok_or("failed to generate nanosecond timestamp")?;
    let start_ns = now_ns - (24 * 60 * 60 * 1_000_000_000_i64);

    let query = if message_type.is_empty() {
        r#"{job=~"mediakraken.*"}"#.to_string()
    } else {
        let escaped_message_type = escape_logql_string(message_type);
        format!(r#"{{job=~"mediakraken.*"}} |= "{}""#, escaped_message_type)
    };

    let resp: LokiResponse = query_client()
        .get(LOKI_QUERY_URL)
        .query(&[
            ("query", query.as_str()),
            ("limit", "100"),
            ("direction", "backward"),
            ("start", &start_ns.to_string()),
            ("end", &now_ns.to_string()),
        ])
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let capacity = resp
        .data
        .result
        .iter()
        .map(|stream| stream.values.len())
        .sum::<usize>();

    let mut logs = Vec::with_capacity(capacity);

    for stream in resp.data.result {
        let stream_str = stream_to_logql(&stream.stream);
        for [ts, line] in stream.values {
            logs.push(LokiLog {
                timestamp_ns: ts.parse()?,
                labels: stream_str.clone(),
                line,
            });
        }
    }

    Ok(logs)
}
