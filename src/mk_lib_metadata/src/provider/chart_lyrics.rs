// http://www.chartlyrics.com/api.aspx

use mk_lib_network;

pub async fn provider_chart_lyrics_fetch(
    artist_name: String,
    song_name: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let json_data = mk_lib_network::mk_lib_network::mk_data_from_url_to_json(format!(
        "http://api.chartlyrics.com/apiv1.asmx/SearchLyricDirect?artist={}&song={}",
        artist_name, song_name
    ))
    .await
    .unwrap();
    Ok(json_data)
}
