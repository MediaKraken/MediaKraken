// https://www.upcitemdb.com/wp/docs/main/development/getting-started/

use mk_lib_database;
use mk_lib_network::mk_lib_network;
use serde_json::json;
use sqlx::types::Uuid;
use std::fmt::Write;

pub async fn provider_upcitemdb_fetch_by_upc(
    sqlx_pool: &sqlx::PgPool,
    upc_code: Vec<&i32>,
    api_token: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // join up upc code for query below
    let mut s = String::new();
    for (i, n) in upc_code.iter().enumerate() {
        if i > 0 {
            s.push_str("%20");
        }
        write!(s, "{}", n).unwrap();
    }
    let url_result = mk_lib_network::mk_data_from_url_to_json(format!(
        "https://api.upcitemdb.com/prod/trial/lookup?upc={}",
        s
    ))
    .await
    .unwrap();
    Ok(url_result)
}
