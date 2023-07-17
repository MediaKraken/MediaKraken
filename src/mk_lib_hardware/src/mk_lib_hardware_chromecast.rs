// https://crates.io/crates/mdns

use mk_lib_logging::mk_lib_logging;
use futures_util::{pin_mut, stream::StreamExt};
use serde_json::json;
use std::time::Duration;
use stdext::function_name;

const CHROMECAST_SERVICE_NAME: &'static str = "_googlecast._tcp.local";

pub async fn mk_hardware_chromecast_discover(
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let stream = mdns::discover::all(CHROMECAST_SERVICE_NAME, Duration::from_secs(15))?.listen();
    pin_mut!(stream);
    while let Some(Ok(response)) = stream.next().await {
        let addr = response.socket_address();
        let host = response.hostname();
        if let (Some(host), Some(addr)) = (host, addr) {
            #[cfg(debug_assertions)]
            {
                mk_lib_logging::mk_logging_post_elk(
                    std::module_path!(),
                    json!({ "found cast device": host, "at": addr }),
                )
                .await
                .unwrap();
            }
        } else {
            #[cfg(debug_assertions)]
            {
                mk_lib_logging::mk_logging_post_elk(
                    std::module_path!(),
                    json!({ "cast device does not advertise address": "" }),
                )
                .await
                .unwrap();
            }
        }
    }
    Ok(json!({}))
}
