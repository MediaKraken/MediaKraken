use chrono::prelude::*;

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
            "http://mkelk.beaverbay.local:9200/{}/_doc",
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
