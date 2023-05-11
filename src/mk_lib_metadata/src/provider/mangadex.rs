// https://github.com/edg-l/mangadex-rs

use mangadex::api::manga::*;
use mangadex::schema::manga::*;
use mangadex::schema::LanguageCode;
use mangadex::Client;
use mk_lib_logging::mk_lib_logging;
use serde_json::json;
use std::error::Error;
use stdext::function_name;

pub async fn provider_mangadex_login(
    user_name: String,
    user_password: String,
) -> Result<mangadex::Client, Box<dyn Error>> {
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
    client.login(&user_name, &user_password).await?;
    Ok(client)
}
