use chrono::prelude::*;
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use tokio::time::Duration;

pub async fn mk_logging_post_elk(
    message_type: &str,
    message_text: serde_json::Value,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let utc: DateTime<Utc> = Utc::now();
    let data = serde_json::json!({"@timestamp": utc.format("%Y-%m-%dT%H:%M:%S.%f").to_string(),
        "message": message_text, "type": message_type, "user": {"id": "metaman"}});
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(100);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    let echo_json: serde_json::Value = client
        .post(format!(
            "http://mkstack-elk:9200/{}/_doc",
            std::env::current_exe()
                .expect("Can't get the exec path")
                .file_name()
                .expect("Can't get the exec name")
                .to_string_lossy()
                .into_owned(),
        ))
        .timeout(Duration::from_secs(30))
        .header("Content-Type", "application/json")
        .json(&data)
        .send()
        .await?
        .json()
        .await?;
    Ok(echo_json)
}
