// https://github.com/kolapapa/surge-ping

use std::time::Duration;
use serde_json::json;
use stdext::function_name;

pub async fn mk_lib_network_ping(addr: String) -> Result<Duration, Box<dyn std::error::Error>> {
    let payload = [0; 8];
    let (_packet, duration) = surge_ping::ping(addr.parse()?, &payload).await?;
    //println!("Ping took {:.3?}", duration);
    Ok(duration)
}
