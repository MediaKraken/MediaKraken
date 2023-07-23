// https://imvdb.com/developers/api

use mk_lib_network;
use std::collections::HashMap;

const BASE_API_URL: &str = "http://imvdb.com/api/v1";
const BASE_USER_AGENT: &str = "MediaKraken_0.1.6";

pub async fn provider_imvdb_video_fetch_by_id(
    sqlx_pool: &sqlx::PgPool,
    video_id: i32,
    metadata_uuid: uuid::Uuid,
    api_key: &String,
) -> Result<uuid::Uuid, Box<dyn std::error::Error>> {
    let mut custom_headers: HashMap<String, String> = HashMap::new();
    custom_headers.insert(String::from("User-Agent"), BASE_USER_AGENT.to_string());
    custom_headers.insert(String::from("IMVDB-APP-KEY"), api_key.to_string());
    custom_headers.insert(String::from("Accept"), String::from("application/json"));
    let headers = mk_lib_network::mk_lib_network::custom_headers(&custom_headers).await;
    let _result = mk_lib_network::mk_lib_network::mk_data_from_url_to_json_custom_headers(
        format!(
            "{}/video/{}?include=sources,credits,bts,featured,popularity,countries",
            BASE_API_URL, video_id
        ),
        headers,
    )
    .await.unwrap();
    let metadata_uuid = uuid::Uuid::nil(); // so not found checks verify later
    Ok(metadata_uuid)
}

pub async fn provider_imvdb_search_video_by_names(
    band_name: String,
    song_name: String,
    api_key: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut custom_headers: HashMap<String, String> = HashMap::new();
    custom_headers.insert(String::from("User-Agent"), BASE_USER_AGENT.to_string());
    custom_headers.insert(String::from("IMVDB-APP-KEY"), api_key);
    custom_headers.insert(String::from("Accept"), String::from("application/json"));
    let headers = mk_lib_network::mk_lib_network::custom_headers(&custom_headers).await;
    let _result = mk_lib_network::mk_lib_network::mk_data_from_url_to_json_custom_headers(
        format!(
            "{}/search/videos?q=/{}+{}",
            BASE_API_URL, band_name.replace(" ", "+"), song_name.replace(" ", "+")
        ),
        headers,
    )
    .await;
    Ok(())
}

pub async fn provider_imvdb_search_videos_by_band(
    band_name: String,
    api_key: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut custom_headers: HashMap<String, String> = HashMap::new();
    custom_headers.insert(String::from("User-Agent"), BASE_USER_AGENT.to_string());
    custom_headers.insert(String::from("IMVDB-APP-KEY"), api_key);
    custom_headers.insert(String::from("Accept"), String::from("application/json"));
    let headers = mk_lib_network::mk_lib_network::custom_headers(&custom_headers).await;
    let _result = mk_lib_network::mk_lib_network::mk_data_from_url_to_json_custom_headers(
        format!(
            "{}/search/entities?q=/{}",
            BASE_API_URL, band_name.replace(" ", "+")
        ),
        headers,
    )
    .await;
    Ok(())
}

/*
pub async fn meta_fetch_save_imvdb(db_connection, imvdb_id, metadata_uuid):
    """
    # fetch from imvdb
    """
    // fetch and save json data via tmdb id
    result_json = await common_global.api_instance.com_imvdb_video_info(imvdb_id)
    if result_json.status_code == 200:
        // set and insert the record
        await db_connection.db_meta_music_video_add(metadata_uuid,
                                                    {'imvdb': str(result_json['id'])},
                                                    result_json['artists'][0]['slug'],
                                                    result_json['song_slug'],
                                                    result_json,
                                                    None)
 */
