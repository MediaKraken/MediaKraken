use chrono::prelude::*;
use elasticsearch::cert::CertificateValidation;
use elasticsearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use elasticsearch::{Elasticsearch, IndexParts};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use std::sync::OnceLock;
use tokio::time::Duration;
use url::Url;

const ELK_POST_URL: &str =
    "http://elasticsearch-es-http.elastic-stack.svc.mkcluster.local:9200/mklogs/_doc";
const ELK_HTTPS_URL: &str = "https://elasticsearch-es-http.elastic-stack.svc.mkcluster.local:9200";

fn elk_payload(
    message_type: &str,
    message_module: &str,
    message_text: serde_json::Value,
) -> serde_json::Value {
    let utc: DateTime<Utc> = Utc::now();
    serde_json::json!({
        "@timestamp": utc.format("%Y-%m-%dT%H:%M:%S.%f").to_string(),
        "type": message_type,
        "message": message_text,
        "module": message_module,
        "user": {"id": "mediakraken"}
    })
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

fn insecure_reqwest_client() -> Result<&'static reqwest::Client, Box<dyn std::error::Error>> {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

    // Check if client is already initialized
    if let Some(client) = CLIENT.get() {
        return Ok(client);
    }

    // Initialize the client
    let built = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    // Try to set the client - if this fails, we'll return an error
    CLIENT.set(built).map_err(|_| {
        "Failed to initialize insecure reqwest client: client already initialized".to_string()
    })?;

    // Return the client
    CLIENT.get().ok_or(
        "failed to retrieve insecure reqwest client after initialization"
            .to_string()
            .into(),
    )
}

fn insecure_elasticsearch_client() -> Result<&'static Elasticsearch, Box<dyn std::error::Error>> {
    static CLIENT: OnceLock<Elasticsearch> = OnceLock::new();

    // Check if client is already initialized
    if let Some(client) = CLIENT.get() {
        return Ok(client);
    }

    // Initialize the client
    let url = Url::parse(ELK_HTTPS_URL)?;
    let conn_pool = SingleNodeConnectionPool::new(url);
    let transport = TransportBuilder::new(conn_pool)
        .cert_validation(CertificateValidation::None)
        .build()?;

    // Try to set the client - if this fails, we'll return an error
    CLIENT.set(Elasticsearch::new(transport)).map_err(|_| "Failed to initialize insecure elasticsearch client: client already initialized".to_string())?;

    // Return the client
    CLIENT.get().ok_or(
        "failed to retrieve insecure elasticsearch client after initialization"
            .to_string()
            .into(),
    )
}

pub async fn mk_logging_post_elk_retry(
    message_type: &str,
    message_module: &str,
    message_text: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = elk_payload(message_type, message_module, message_text);

    retrying_client()
        .post(ELK_POST_URL)
        .timeout(Duration::from_secs(30))
        .header("Content-Type", "application/json")
        .json(&data)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

pub async fn mk_logging_post_elk_ignore_ssl(
    message_type: &str,
    message_module: &str,
    message_text: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = elk_payload(message_type, message_module, message_text);

    insecure_reqwest_client()?
        .post(ELK_POST_URL)
        .timeout(Duration::from_secs(30))
        .header("Content-Type", "application/json")
        .json(&data)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}

pub async fn mk_logging_post_elk_lib(
    message_type: &str,
    message_module: &str,
    message_text: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = elk_payload(message_type, message_module, message_text);

    insecure_elasticsearch_client()?
        .index(IndexParts::Index("mklogs"))
        .body(data)
        .send()
        .await?
        .error_for_status_code()?;

    Ok(())
}
