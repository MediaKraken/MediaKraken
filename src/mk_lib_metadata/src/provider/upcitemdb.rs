// https://www.upcitemdb.com/wp/docs/main/development/getting-started/

use mk_lib_database;
use mk_lib_network::mk_lib_network;
use serde_json::json;
use sqlx::types::Uuid;

pub async fn provider_upcitemdb_fetch_by_upc(
    sqlx_pool: &sqlx::PgPool,
    upc_code: Vec<&str>,
    api_token: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url_result = mk_lib_network::mk_data_from_url_to_json(format!(
        "https://api.upcitemdb.com/prod/trial/lookup?upc={}",
        upc_code.join("%20")
    ))
    .await
    .unwrap();
    Ok(url_result)
}
