// https://www.upcitemdb.com/wp/docs/main/development/getting-started/

use mk_lib_network::mk_lib_network;
use std::fmt::Write;

pub async fn provider_upcitemdb_fetch_by_upc(
    _sqlx_pool: &sqlx::PgPool,
    upc_code: Vec<&str>,
    _api_token: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // join up upc code for query below
    let mut s = String::new();
    for (i, n) in upc_code.iter().enumerate() {
        if i > 0 {
            s.push_str("%20");
        }
        write!(s, "{}", n).map_err(|e| format!("write failed: {e}"))?;
    }
    let url_result = mk_lib_network::mk_data_from_url_to_json(format!(
        "https://api.upcitemdb.com/prod/trial/lookup?upc={}",
        s
    ))
    .await?;
    Ok(url_result)
}
