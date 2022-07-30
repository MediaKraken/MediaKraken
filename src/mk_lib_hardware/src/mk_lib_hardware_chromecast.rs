#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://crates.io/crates/mdns

use futures_util::{pin_mut, stream::StreamExt};
use mdns::Error;
use std::time::Duration;
use serde_json::json;

const CHROMECAST_SERVICE_NAME: &'static str = "_googlecast._tcp.local";

pub async fn mk_hardware_chromecast_discover() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let stream = mdns::discover::all(CHROMECAST_SERVICE_NAME, Duration::from_secs(15))?.listen();
    pin_mut!(stream);
    while let Some(Ok(response)) = stream.next().await {
        let addr = response.socket_address();
        let host = response.hostname();
        if let (Some(host), Some(addr)) = (host, addr) {
            println!("found cast device {} at {}", host, addr);
        } else {
            println!("cast device does not advertise address");
        }
    }
    Ok(json!({}))
}
