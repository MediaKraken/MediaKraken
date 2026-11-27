// https://ean-db.com/docs

use mk_lib_network::mk_lib_network;
use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;

const EAN_DB_BASE_URL: &str = "https://ean-db.com/api";

async fn ean_db_bearer_headers(
    api_token: &str,
) -> Result<reqwest::header::HeaderMap, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert(
        "authorization".to_string(),
        format!("Bearer {}", api_token.trim()),
    );
    map.insert("accept".to_string(), "application/json".to_string());
    Ok(mk_lib_network::custom_headers(&map).await)
}

/// Look up a product by barcode. 404s (no charge) when the barcode is unknown.
pub async fn provider_ean_db_product(
    _sqlx_pool: &sqlx::PgPool,
    barcode: &str,
    api_token: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("{}/v2/product/{}", EAN_DB_BASE_URL, barcode.trim());
    let headers = ean_db_bearer_headers(api_token).await?;
    mk_lib_network::mk_data_from_url_to_json_custom_headers(url, headers).await
}

/// Get the authenticated account details (primary email + credit balance).
pub async fn provider_ean_db_account(
    _sqlx_pool: &sqlx::PgPool,
    api_token: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("{}/v2/account", EAN_DB_BASE_URL);
    let headers = ean_db_bearer_headers(api_token).await?;
    mk_lib_network::mk_data_from_url_to_json_custom_headers(url, headers).await
}

/// Issue a new API JWT. Up to 3 JWTs can exist per account; a 409 is returned
/// once the limit is reached (revoke one first).
pub async fn provider_ean_db_jwt_create(
    _sqlx_pool: &sqlx::PgPool,
    api_token: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("{}/v2/account/jwt", EAN_DB_BASE_URL);
    let client = reqwest::Client::builder().build()?;
    let response = client
        .put(url)
        .timeout(Duration::from_secs(30))
        .bearer_auth(api_token.trim())
        .send()
        .await?;
    let status = response.status();
    let body: serde_json::Value = response.json().await?;
    if !status.is_success() {
        return Err(format!("ean-db JWT create failed with status {}: {}", status, body).into());
    }
    Ok(body)
}

/// Revoke an API JWT. Returns 204 No Content on success.
pub async fn provider_ean_db_jwt_revoke(
    _sqlx_pool: &sqlx::PgPool,
    jwt: &str,
    api_token: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("{}/v2/account/jwt", EAN_DB_BASE_URL);
    let client = reqwest::Client::builder().build()?;
    let response = client
        .delete(url)
        .timeout(Duration::from_secs(30))
        .bearer_auth(api_token.trim())
        .json(&json!({ "jwt": jwt }))
        .send()
        .await?;
    let status = response.status();
    if status.as_u16() == 204 {
        return Ok(serde_json::Value::Null);
    }
    let body: serde_json::Value = response.json().await?;
    if !status.is_success() {
        return Err(format!("ean-db JWT revoke failed with status {}: {}", status, body).into());
    }
    Ok(body)
}
