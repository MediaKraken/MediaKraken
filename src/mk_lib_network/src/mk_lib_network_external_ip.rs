#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use stdext::function_name;
use serde_json::json;

pub async fn mk_lib_network_external_ip() {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let response = reqwest::get("https://myexternalip.com/raw").await?;
    let content = response.bytes().await?;
    Ok(str::from_utf8(&content).unwrap().to_string())
}

/*
"https://icanhazip.com/",
"https://myexternalip.com/raw",
"https://ifconfig.io/ip",
"https://ipecho.net/plain",
"https://checkip.amazonaws.com/",
"https://ident.me/",
"http://whatismyip.akamai.com/",
"https://myip.dnsomatic.com/",
"https://diagnostic.opendns.com/myip",
 */
