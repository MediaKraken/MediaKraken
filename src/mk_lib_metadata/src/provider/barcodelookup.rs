// https://www.barcodelookup.com/api

use mk_lib_network::mk_lib_network;

pub async fn provider_barcodelookup_fetch(
    _sqlx_pool: &sqlx::PgPool,
    upc_code: &i32,
    api_token: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result = mk_lib_network::mk_data_from_url_to_json(format!(
        "https://api.barcodelookup.com/v3/products?barcode={}&formatted=y&key={}",
        upc_code, api_token
    ))
    .await?;

    Ok(url_result)
}
