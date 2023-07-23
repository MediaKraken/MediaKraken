// https://github.com/edg-l/mangadex-rs

use mangadex::Client;
use serde_json::json;
use std::error::Error;
use stdext::function_name;

pub async fn provider_mangadex_login(
    user_name: String,
    user_password: String,
) -> Result<mangadex::Client, Box<dyn Error>> {
    let mut client = Client::default();
    client.login(&user_name, &user_password).await?;
    Ok(client)
}
