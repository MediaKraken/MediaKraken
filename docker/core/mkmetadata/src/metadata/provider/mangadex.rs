#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/edg-l/mangadex-rs
// mangadex = "0.1.2"

use mangadex::api::manga::*;
use mangadex::schema::manga::*;
use mangadex::schema::LanguageCode;
use mangadex::Client;

pub async fn provider_mangadex_login(
    user_name: String,
    user_password: String,
) -> Result<mangadex::Client, sqlx::Error> {
    let mut client = Client::default();
    client.login(user_name, user_password).await?;
    Ok(client)
}
