// https://www.giantbomb.com/api/

use mk_lib_network::mk_lib_network;

pub async fn mk_provider_giant_bomb_platforms(
    api_key: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // NOTE: giant_bomb only accepts the api key as a query parameter; it is
    // visible in proxy/server logs. Treat the key as low-sensitivity and rotate
    // regularly.
    let url_result = mk_lib_network::mk_data_from_url_to_json(format!(
        "https://www.giantbomb.com/api/platforms/?api_key={}",
        api_key
    ))
    .await?;
    Ok(url_result)
}
