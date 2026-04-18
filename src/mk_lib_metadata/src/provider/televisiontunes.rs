use mk_lib_network::mk_lib_network;
use std::error::Error;
use url::form_urlencoded;

pub async fn provider_televisiontunes_theme_fetch(
    tv_show_name: String,
    tv_show_theme_path: String,
) -> Result<uuid::Uuid, Box<dyn Error>> {
    let base_url = "https://www.televisiontunes.com/";
    let slug: String =
        form_urlencoded::byte_serialize(tv_show_name.replace(' ', "_").as_bytes()).collect();
    let show_url = format!("{}{}", base_url, slug);
    let response = reqwest::get(&show_url).await?;
    if !response.status().is_success() {
        return Ok(uuid::Uuid::nil());
    }
    let body = response.text().await?;
    const HREF_MARKER: &str = "href=\"/song/download/";
    let dl_position = match body.find(HREF_MARKER) {
        Some(pos) => pos,
        None => return Ok(uuid::Uuid::nil()),
    };
    let tail = &body[dl_position + HREF_MARKER.len()..];
    let dl_end_position = match tail.find('"') {
        Some(pos) => pos,
        None => return Ok(uuid::Uuid::nil()),
    };
    let dl_id = &tail[..dl_end_position];
    let dl_url = format!("{}song/download/{}", base_url, dl_id);
    mk_lib_network::mk_download_file_from_url(dl_url, &tv_show_theme_path).await?;
    Ok(uuid::Uuid::now_v7())
}
