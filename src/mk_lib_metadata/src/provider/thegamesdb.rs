// https://api.thegamesdb.net/
// https://cdn.thegamesdb.net/json/database-latest.json - db dump

use mk_lib_network::mk_lib_network;
use std::error::Error;

pub async fn thegamesdb_database_fetch() -> Result<serde_json::Value, Box<dyn Error>> {
    let json_data = mk_lib_network::mk_data_from_url_to_json(
        "https://cdn.thegamesdb.net/json/database-latest.json".to_string(),
    )
    .await?;
    Ok(json_data)
}

pub async fn thegamesdb_platforms_read(
    api_key: String,
) -> Result<serde_json::Value, Box<dyn Error>> {
    // note: the two query params must be joined with '&', not '?'
    let url = format!(
        "https://api.thegamesdb.net/v1/Platforms?apikey={}&fields=icon,console,controller,developer,manufacturer,media,cpu,memory,graphics,sound,maxcontrollers,display,overview,youtube",
        api_key
    );
    mk_lib_network::mk_data_from_url_to_json(url).await
}

pub async fn thegamesdb_games_updated(
    api_key: String,
    edit_id: String,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let url = format!(
        "https://api.thegamesdb.net/v1/Games/Updates?apikey={}&last_edit_id={}",
        api_key, edit_id
    );
    mk_lib_network::mk_data_from_url_to_json(url).await
}

pub async fn thegamesdb_genre_read(api_key: String) -> Result<serde_json::Value, Box<dyn Error>> {
    let url = format!("https://api.thegamesdb.net/v1/Genres?apikey={}", api_key);
    mk_lib_network::mk_data_from_url_to_json(url).await
}

pub async fn thegamesdb_developers_read(
    api_key: String,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let url = format!(
        "https://api.thegamesdb.net/v1/Developers?apikey={}",
        api_key
    );
    mk_lib_network::mk_data_from_url_to_json(url).await
}

pub async fn thegamesdb_publishers_read(
    api_key: String,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let url = format!(
        "https://api.thegamesdb.net/v1/Publishers?apikey={}",
        api_key
    );
    mk_lib_network::mk_data_from_url_to_json(url).await
}
