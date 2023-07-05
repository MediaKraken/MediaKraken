// https://musicbrainz.org/doc/Cover_Art_Archive/API

use mk_lib_network;
use std::collections::HashMap;

const BASE_API_URL: &str = "http://coverartarchive.org/release";
const BASE_USER_AGENT: &str = "MediaKraken_0.1.6";

pub async fn provider_coverartarchive_by_release_guid(
    metadata_uuid: uuid::Uuid,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut custom_headers: HashMap<String, String> = HashMap::new();
    custom_headers.insert(String::from("User-Agent"), BASE_USER_AGENT.to_string());
    custom_headers.insert(String::from("Accept"), String::from("application/json"));
    let headers = mk_lib_network::mk_lib_network::custom_headers(&custom_headers).await;
    let result = mk_lib_network::mk_lib_network::mk_data_from_url_to_json_custom_headers(
        format!("{}/{}/", BASE_API_URL, metadata_uuid),
        headers,
    )
    .await
    .unwrap();
    Ok(())
}
