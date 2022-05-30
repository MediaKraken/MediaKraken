#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use chrono::prelude::*;

pub async fn mk_logging_post_elk(message_type: &str,
                                 message_text: serde_json::Value,
                                 index_name: &str) {
    let utc: DateTime<Utc> = Utc::now();
    let data = serde_json::json!({"@timestamp": utc.format("%Y-%m-%dT%H:%M:%S.%f").to_string(),
        "message": message_text, "type": message_type, "user": {"id": "metaman"}});
    let client = reqwest::Client::new();
    let _resp = client.post(format!("http://mkelk:9200/{}/MediaKraken",
                                   index_name))
        .header("Content-Type", "application/json")
        .json(&data)
        .send()
        .await;
}
