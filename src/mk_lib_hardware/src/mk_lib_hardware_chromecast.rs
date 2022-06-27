#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://crates.io/crates/mdns

use mdns::{Record, RecordKind};

const CHROMECAST_SERVICE_NAME: &'static str = "_googlecast._tcp.local";

pub async fn mk_hardware_chromecast_discover()
                                    -> Result<serde_json::Value,
                                    Box<dyn std::error::Error>> {
    let stream = mdns::discover::all(CHROMECAST_SERVICE_NAME, Duration::from_secs(15))?.listen();
    pin_mut!(stream);
    while let Some(Ok(response)) = stream.next().await {
        let addr = response.records()
                           .filter_map(self::to_ip_addr)
                           .next();
        if let Some(addr) = addr {
            println!("found cast device at {}", addr);
        } else {
            println!("cast device does not advertise address");
        }
    }
    Ok(res)
}