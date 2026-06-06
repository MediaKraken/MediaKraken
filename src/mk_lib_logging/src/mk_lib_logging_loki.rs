use chrono::Utc;
use reqwest::{Client, Url};
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

// FIX 1: Add `status` and optional `error` + `data` fields.
// Loki returns either:
//   {"status":"success","data":{...}}
// or:
//   {"status":"error","error":"some message"}
// Without `status` the code could not detect Loki-level errors, and the missing
// `data` field would cause a cryptic serde deserialization error.
#[derive(Debug, Deserialize)]
struct LokiResponse {
    status: String,
    #[serde(default)]
    error: Option<String>,
    // FIX 5: `data` is now Option so that Loki error responses (which omit the
    // field entirely) deserialize cleanly instead of failing with an opaque
    // "missing field `data`" message.
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
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(100);
        ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build()
    })
}

#[expect(dead_code)]
fn query_client() -> &'static Client {
    // Keep this function in case callers outside this module still reference it.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_to_bytes_short_string() {
        let s = "hello";
        assert_eq!(truncate_to_bytes(s, 100), "hello");
    }

    #[test]
    fn test_truncate_to_bytes_exact_length() {
        let s = "hello";
        assert_eq!(truncate_to_bytes(s, 5), "hello");
    }

    #[test]
    fn test_truncate_to_bytes_longer_than_max() {
        let s = "hello world";
        let result = truncate_to_bytes(s, 5);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_truncate_to_bytes_multibyte_char_boundary() {
        let s = "hello \u{00e9}world";
        let result = truncate_to_bytes(s, 6);
        assert!(result.is_char_boundary(6));
    }

    #[test]
    fn test_truncate_to_bytes_empty() {
        assert_eq!(truncate_to_bytes("", 10), "");
    }

    #[test]
    fn test_stream_labels_to_string_empty() {
        let labels = HashMap::new();
        assert_eq!(stream_labels_to_string(&labels), "");
    }

    #[test]
    fn test_stream_labels_to_string_single() {
        let mut labels = HashMap::new();
        labels.insert("job".to_string(), "mediakraken".to_string());
        let result = stream_labels_to_string(&labels);
        assert_eq!(result, r#"job="mediakraken""#);
    }

    #[test]
    fn test_stream_labels_to_string_multiple_sorted() {
        let mut labels = HashMap::new();
        labels.insert("zebra".to_string(), "z".to_string());
        labels.insert("alpha".to_string(), "a".to_string());
        let result = stream_labels_to_string(&labels);
        assert!(result.starts_with(r#"alpha="a""#));
        assert!(result.contains(r#"zebra="z""#));
    }

    #[test]
    fn test_stream_to_logql_empty() {
        let labels = HashMap::new();
        assert_eq!(stream_to_logql(&labels), "{}");
    }

    #[test]
    fn test_stream_to_logql_single() {
        let mut labels = HashMap::new();
        labels.insert("job".to_string(), "mediakraken".to_string());
        assert_eq!(stream_to_logql(&labels), r#"{job="mediakraken"}"#);
    }

    #[test]
    fn test_escape_logql_string_no_escapes() {
        assert_eq!(escape_logql_string("hello world"), "hello world");
    }

    #[test]
    fn test_escape_logql_string_backslash() {
        assert_eq!(escape_logql_string(r"hello\world"), r"hello\\world");
    }

    #[test]
    fn test_escape_logql_string_quotes() {
        assert_eq!(escape_logql_string(r#"hello"world"#), r#"hello\"world"#);
    }

    #[test]
    fn test_escape_logql_string_both() {
        let input = r#"hello\"world"#;
        let result = escape_logql_string(input);
        assert_eq!(result, r#"hello\\"world"#);
    }

    #[test]
    fn test_loki_log_struct() {
        let log = LokiLog {
            timestamp_ns: 1234567890123456789,
            labels: "job=\"mediakraken\"".to_string(),
            line: "test log line".to_string(),
        };
        assert_eq!(log.timestamp_ns, 1234567890123456789);
        assert_eq!(log.labels, "job=\"mediakraken\"");
        assert_eq!(log.line, "test log line");
    }

    #[test]
    fn test_loki_max_entry_bytes_constant() {
        assert_eq!(LOKI_MAX_ENTRY_BYTES, 262_144);
    }

    #[test]
    fn test_loki_safe_entry_bytes_constant() {
        assert_eq!(LOKI_SAFE_ENTRY_BYTES, 240_000);
    }

    #[test]
    fn test_loki_safe_entry_bytes_less_than_max() {
        assert!(LOKI_SAFE_ENTRY_BYTES < LOKI_MAX_ENTRY_BYTES);
    }
}

pub async fn mk_logging_loki_read(
    message_type: &str,
) -> Result<Vec<LokiLog>, Box<dyn std::error::Error>> {
    let now_ns = Utc::now()
        .timestamp_nanos_opt()
        .ok_or("failed to generate nanosecond timestamp")?;

    let start_ns = now_ns - (24 * 60 * 60 * 1_000_000_000_i64);

    // FIX 4: `message_type` is the name of an app/job — it should filter via
    // a label selector (`app="<value>"`), not a log-line substring filter
    // (`|= "<value>"`).  Using `|=` would search the raw JSON content of every
    // log line for the string, which is both slower and semantically wrong when
    // the intent is to scope reads to a specific MediaKraken service.
    let query = if message_type.is_empty() {
        r#"{job="mediakraken"}"#.to_string()
    } else {
        let escaped = escape_logql_string(message_type);
        // Label-based filter: returns only streams whose `app` label matches.
        format!(r#"{{job="mediakraken",app="{}"}}"#, escaped)
    };

    // FIX 2 & 3: Use `retrying_client()` (exponential back-off, same as push)
    // and add a 30-second timeout so the caller is never blocked indefinitely.
    // `reqwest_middleware::RequestBuilder` does not expose `.query()`, so build
    // the URL with query parameters up front via `Url::parse_with_params`.
    let start_str = start_ns.to_string();
    let end_str = now_ns.to_string();
    let url = Url::parse_with_params(
        LOKI_QUERY_URL,
        &[
            ("query", query.as_str()),
            ("limit", "100"),
            ("direction", "backward"),
            ("start", start_str.as_str()),
            ("end", end_str.as_str()),
        ],
    )?;

    let resp: LokiResponse = retrying_client()
        .get(url)
        .timeout(Duration::from_secs(30))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    // FIX 1 (cont.): Surface the Loki-level error instead of silently returning
    // an empty result set or panicking on the missing `data` field.
    if resp.status != "success" {
        let loki_err = resp
            .error
            .unwrap_or_else(|| "unknown loki error".to_string());
        return Err(format!("loki query failed: {}", loki_err).into());
    }

    // FIX 5 (cont.): `data` is now `Option<LokiData>`; unwrap it after the
    // status check — at this point `status == "success"` so `data` is present.
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
            // Improved timestamp parsing with better error handling
            let timestamp_ns: i128 = ts
                .parse()
                .map_err(|e| format!("Failed to parse timestamp '{}': {}", ts, e))?;
            logs.push(LokiLog {
                timestamp_ns,
                labels: stream_str.clone(),
                line,
            });
        }
    }

    Ok(logs)
}
