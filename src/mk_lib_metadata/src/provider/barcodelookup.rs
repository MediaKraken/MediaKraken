// https://www.barcodelookup.com/api

use mk_lib_database;
use mk_lib_network::mk_lib_network;
use serde_json::json;
use sqlx::types::Uuid;

pub async fn provider_barcodelookup_fetch(
    sqlx_pool: &sqlx::PgPool,
    upc_code: &str,
    api_token: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result = mk_lib_network::mk_data_from_url_to_json(format!(
        "TODO fake {} {}",
        api_token,
        upc_code
    ))
    .await
    .unwrap();
    Ok(url_result)
}
