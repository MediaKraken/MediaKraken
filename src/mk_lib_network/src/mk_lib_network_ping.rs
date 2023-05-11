// https://github.com/kolapapa/surge-ping

use std::time::Duration;

use mk_lib_logging::mk_lib_logging;
use serde_json::json;
use stdext::function_name;
use surge_ping::{Client, Config, PingIdentifier, PingSequence, ICMP};

pub async fn mk_lib_network_ping(addr: String) -> Result<Duration, Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let payload = [0; 8];
    let (_packet, duration) = surge_ping::ping(addr.parse()?, &payload).await?;
    //println!("Ping took {:.3?}", duration);
    Ok(duration)
}
