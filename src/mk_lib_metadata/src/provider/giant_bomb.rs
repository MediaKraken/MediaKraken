#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://www.giantbomb.com/api/

use std::error::Error;

#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;

pub async fn mk_provider_giant_bomb_platforms(
    api_key: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result: serde_json::Value = mk_lib_network::mk_data_from_url_to_json(format!(
        "https://www.giantbomb.com/api/platforms/?api_key={}",
        api_key
    ))
    .await
    .unwrap();
    Ok(url_result)
}
