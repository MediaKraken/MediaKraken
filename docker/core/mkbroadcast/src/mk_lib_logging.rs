#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use chrono::prelude::*;
use std::env;

/*
Since we created the document using the HTTP PUT method, we needed to set the id of the document, otherwise, we will get an error.

If you don’t have a unique id, you can use HTTP POST then Elasticsearch will create a unique id for you.
*/
pub async fn mk_logging_post_elk(
    message_type: &str,
    message_text: serde_json::Value,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let utc: DateTime<Utc> = Utc::now();
    let data = serde_json::json!({"@timestamp": utc.format("%Y-%m-%dT%H:%M:%S.%f").to_string(),
        "message": message_text, "type": message_type, "user": {"id": "metaman"}});
    let client = reqwest::Client::new();
    let echo_json = client
        .post(format!(
            "http://mkelk:9200/{}/_doc",
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
    // println!("{:#?}", data);
    // println!("{:#?}", echo_json);
    Ok(echo_json)
}
