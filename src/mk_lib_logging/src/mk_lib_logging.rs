use chrono::prelude::*;
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::RetryTransientMiddleware;

pub async fn mk_logging_post_elk(
    message_type: &str,
    message_text: serde_json::Value,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let utc: DateTime<Utc> = Utc::now();
    let data = serde_json::json!({"@timestamp": utc.format("%Y-%m-%dT%H:%M:%S.%f").to_string(),
        "message": message_text, "type": message_type, "user": {"id": "metaman"}});
    let retry_policy = reqwest_retry::policies::ExponentialBackoff {
        // How many times the policy will tell the middleware to retry the request.
        max_n_retries: 100,
        min_retry_interval: std::time::Duration::from_secs(30),
        max_retry_interval: std::time::Duration::from_secs(300),
        backoff_exponent: 2,
        //jitter:  retry_policies::Jitter::Bounded,
    };
    let retry_transient_middleware = RetryTransientMiddleware::new_with_policy(retry_policy);
    let client = ClientBuilder::new(Client::new())
        .with(retry_transient_middleware)
        .build();
    //let client = reqwest::Client::new();
    let echo_json = client
        .post(format!(
            "http://mkstack_elk:9200/{}/_doc",
            std::env::current_exe()
                .expect("Can't get the exec path")
                .file_name()
                .expect("Can't get the exec name")
                .to_string_lossy()
                .into_owned(),
        ))
        .header("Content-Type", "application/json")
        .json(&data)
        .send()
        .await?
        .json()
        .await?;
    //println!("{:#?}", data);
    //println!("{:#?}", echo_json);
    Ok(echo_json)
}
