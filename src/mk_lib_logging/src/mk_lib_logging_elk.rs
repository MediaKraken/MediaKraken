use chrono::prelude::*;
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use tokio::time::Duration;
use elasticsearch::{
    Elasticsearch, Error,
    http::transport::Transport,
    http::transport::TransportBuilder,
    IndexParts,
    cert::CertificateValidation,
};
use elasticsearch::http::transport::SingleNodeConnectionPool;
use elasticsearch::http::Method;
use elasticsearch::SearchParts;
use elasticsearch::http::headers::HeaderMap;
use serde_json::Value;
use serde_json::json;
use url::Url;

pub async fn mk_logging_post_elk_retry(
    message_type: &str,
    message_module: &str,
    message_text: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let utc: DateTime<Utc> = Utc::now();
    let data = serde_json::json!({"@timestamp": utc.format("%Y-%m-%dT%H:%M:%S.%f").to_string(),
        "type": message_type, "message": message_text, "module": message_module, "user": {"id": "mediakraken"}});
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(100);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    // the internal url for below is health checks/etc
    let response = client
        .post("http://elasticsearch-es-http.elastic-stack.svc.mkcluster.local:9200/mklogs/_doc")
        .timeout(Duration::from_secs(30))
        .header("Content-Type", "application/json")
        .json(&data)
        .send()
        .await?;
    Ok(())
}

pub async fn mk_logging_post_elk_ignore_ssl(
    message_type: &str,
    message_module: &str,
    message_text: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let utc: DateTime<Utc> = Utc::now();
    let data = serde_json::json!({"@timestamp": utc.format("%Y-%m-%dT%H:%M:%S.%f").to_string(),
        "type": message_type, "message": message_text, "module": message_module, "user": {"id": "mediakraken"}});
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    // the internal url for below is health checks/etc
    let response = client
        .post("http://elasticsearch-es-http.elastic-stack.svc.mkcluster.local:9200/mklogs/_doc")
        .timeout(Duration::from_secs(30))
        .header("Content-Type", "application/json")
        .json(&data)
        .send()
        .await?;
    Ok(())
}

pub async fn mk_logging_post_elk_lib(
    message_type: &str,
    message_module: &str,
    message_text: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let utc: DateTime<Utc> = Utc::now();
    let data = serde_json::json!({"@timestamp": utc.format("%Y-%m-%dT%H:%M:%S.%f").to_string(),
        "type": message_type, "message": message_text, "module": message_module, "user": {"id": "mediakraken"}});
    let url = Url::parse("https://elasticsearch-es-http.elastic-stack.svc.mkcluster.local:9200")?;
    let conn_pool = SingleNodeConnectionPool::new(url);
    let transport = TransportBuilder::new(conn_pool)
        .cert_validation(CertificateValidation::None) // 👈 allow self-signed certs
        .build()
        .unwrap();
    let client = Elasticsearch::new(transport);
    let response = client   
        .index(IndexParts::Index("mklogs"))
        .body(data)
        .send()
        .await?;
    Ok(())
}

/*
debug true
TMDB here2
http error
thread 'tokio-runtime-worker' (24) panicked at src/main.rs:136:26:
called `Result::unwrap()` on an `Err` value: Error { kind: Http(reqwest::Error { kind: Request, url: "http://elasticsearch-es-http.elastic-stack.svc.mkcluster.local:9200/mklogs/_doc", source: hyper_util::client::legacy::Error(SendRequest, hyper::Error(IncompleteMessage)) }) }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/