// https://devapi.barcodespider.com/

use mk_lib_network::mk_lib_network;

const BASE_API_URL: &str = "https://api.barcodespider.com/v1/lookup";

pub async fn provider_barcodespider_fetch_by_upc(
    _sqlx_pool: &sqlx::PgPool,
    upc_code: &str,
    api_token: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let api_response = mk_lib_network::mk_data_from_url_to_json(format!(
        "{}?token={}&upc={}",
        BASE_API_URL, api_token, upc_code
    ))
    .await?;

    // Keep API semantics explicit for callers: Barcodespider returns
    // `item_response.code` + `status` in JSON even for non-success cases.
    if api_response["item_response"]["code"] != 200 {
        return Err(format!(
            "barcodespider lookup failed: {}",
            api_response["item_response"]["message"]
                .as_str()
                .unwrap_or("unknown error")
        )
        .into());
    }

    Ok(api_response)
}
