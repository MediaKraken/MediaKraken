#![cfg_attr(debug_assertions, allow(dead_code))]

// https://github.com/edg-l/mangadex-rs
// mangadex = "0.1.2"

use mangadex::api::manga::*;
use mangadex::schema::manga::*;
use mangadex::schema::LanguageCode;
use mangadex::Client;
use serde_json::json;
use stdext::function_name;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

pub async fn provider_mangadex_login(
    user_name: String,
    user_password: String,
) -> Result<mangadex::Client, sqlx::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let mut client = Client::default();
    client.login(user_name, user_password).await?;
    Ok(client)
}
