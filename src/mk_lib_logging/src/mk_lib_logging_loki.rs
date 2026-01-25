use chrono::prelude::*;
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use tokio::time::Duration;
use std::env;
use std::path::Path;
use serde_json::json;

pub async fn mk_logging_loki_push(
    message_text: serde_json::Value,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("TMDB here3");
    let exe_path = env::current_exe()?;
    let exe_name = exe_path.file_name()
                           .and_then(std::ffi::OsStr::to_str)
                           .ok_or("Invalid executable name")?;
    let log_line = serde_json::to_string(&message_text)?;
    let timestamp_ns = Utc::now().timestamp_nanos().to_string();
    let payload = json!({
            "streams": [
                {
                    "stream": {
                        "mediakraken": exe_name,
                        "format": "json"
                    },
                    "values": [
                        [ timestamp_ns, log_line ]
                    ]
                }
            ]
        });
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(100);
    println!("TMDB here5");
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    println!("TMDB here6");
    let response = client
        .post("http://loki-headless.monitoring.svc.mkcluster.local:3100/loki/api/v1/push")
        .timeout(Duration::from_secs(30))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;
    println!("Loki response status: {}", response.status());
    Ok(())
}

/*
    "streams": [{
      "stream": {"job": "storage-test"},
      "values": [["'"$(date +%s)"'000000000", "storage test message"]]
    }]
 */
