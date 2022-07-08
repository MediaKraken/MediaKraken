#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use sqlx::types::Uuid;
use std::error::Error;
use std::str;
use substring::Substring;

#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;

pub async fn provider_televisiontunes_theme_fetch(
    tv_show_name: String,
    tv_show_theme_path: String,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    let mut metadata_uuid = uuid::Uuid::nil();
    let base_url = "https://www.televisiontunes.com/".to_string();
    let show_url = format!("{}{}", base_url, tv_show_name.replace(" ", "_"));
    let response = reqwest::get(show_url).await?;
    if response.status().is_success() {
        let content = response.bytes().await?;
        let content_string = std::str::from_utf8(&content).unwrap().to_string();
        let dl_position = content_string.find("href=\"/song/download/").unwrap();
        let data_content = content_string.substring(dl_position + 21, dl_position + 50);
        let dl_end_position = data_content.find("\"").unwrap();
        //println!("{:?}", data_content.substring(0, dl_end_position));
        let dl_url = format!(
            "{}{}{}",
            base_url,
            "song/download/",
            data_content.substring(0, dl_end_position)
        );
        //println!("{}", dl_url);
        mk_lib_network::mk_download_file_from_url(dl_url, &tv_show_theme_path);
        metadata_uuid = Uuid::new_v4();
    }
    Ok(metadata_uuid)
}
