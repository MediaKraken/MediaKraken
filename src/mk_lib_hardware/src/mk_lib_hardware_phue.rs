#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use huelib::resource::sensor;
use huelib::{bridge, Bridge};
use serde_json::json;

pub async fn mk_hardware_phue_discover() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let hub_ip_addresses = bridge::discover_nupnp().unwrap();
    for bridge_ip in hub_ip_addresses {
        println!("{}", bridge_ip);
        // Register a new user.
        let username = bridge::register_user(bridge_ip, "huelib-rs example").unwrap();
        let bridge = Bridge::new(bridge_ip, username);
        let lights = bridge.get_all_lights().unwrap();
        println!("{:?}", lights);
    }
    Ok(json!({}))
}
