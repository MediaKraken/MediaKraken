// https://upcmdb.com/api

use mk_lib_network::mk_lib_network;
use std::collections::HashMap;

const UPCMDB_BASE_URL: &str = "https://upcmdb.com/api";

async fn upcmdb_auth_headers(
    api_token: &str,
) -> Result<reqwest::header::HeaderMap, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("x-api-key".to_string(), api_token.to_string());
    map.insert("accept".to_string(), "application/json".to_string());
    Ok(mk_lib_network::custom_headers(&map).await)
}

/// Look up a UPC-A / EAN-12 barcode (12 digits).
pub async fn provider_upcmdb_lookup_by_upc(
    _sqlx_pool: &sqlx::PgPool,
    upc_code: &str,
    api_token: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("{}/v1/lookup/{}", UPCMDB_BASE_URL, upc_code.trim());
    let headers = upcmdb_auth_headers(api_token).await?;
    mk_lib_network::mk_data_from_url_to_json_custom_headers(url, headers).await
}

/// Look up a 13 digit EAN barcode.
pub async fn provider_upcmdb_lookup_by_ean(
    _sqlx_pool: &sqlx::PgPool,
    ean_code: &str,
    api_token: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("{}/v1/lookup/ean/{}", UPCMDB_BASE_URL, ean_code.trim());
    let headers = upcmdb_auth_headers(api_token).await?;
    mk_lib_network::mk_data_from_url_to_json_custom_headers(url, headers).await
}

/// Look up a title by IMDb id (e.g. tt0111161).
pub async fn provider_upcmdb_lookup_by_imdb(
    _sqlx_pool: &sqlx::PgPool,
    imdb_id: &str,
    api_token: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("{}/v1/lookup/imdb/{}", UPCMDB_BASE_URL, imdb_id.trim());
    let headers = upcmdb_auth_headers(api_token).await?;
    mk_lib_network::mk_data_from_url_to_json_custom_headers(url, headers).await
}

/// Search by title (required) and year (optional).
pub async fn provider_upcmdb_search(
    _sqlx_pool: &sqlx::PgPool,
    title: &str,
    year: Option<&str>,
    api_token: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let mut url = url::Url::parse(&format!("{}/v1/search", UPCMDB_BASE_URL))?;
    url.query_pairs_mut().append_pair("title", title.trim());
    if let Some(year) = year.map(str::trim).filter(|year| !year.is_empty()) {
        url.query_pairs_mut().append_pair("year", year);
    }
    let headers = upcmdb_auth_headers(api_token).await?;
    mk_lib_network::mk_data_from_url_to_json_custom_headers(url.to_string(), headers).await
}

/// Public account / API status endpoint (no key required).
pub async fn provider_upcmdb_status(
    _sqlx_pool: &sqlx::PgPool,
    _api_token: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("{}/v1/status", UPCMDB_BASE_URL);
    mk_lib_network::mk_data_from_url_to_json(url).await
}
