#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/nn1ks/huelib-rs
// huelib = "0.13.2"

use huelib::resource::sensor;
use huelib::{bridge, Bridge};
use serde_json::json;

pub async fn mk_hardware_phue_discover() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let hub_ip_addresses = bridge::discover_nupnp().unwrap();
    for bridge_ip in hub_ip_addresses {
        #[cfg(debug_assertions)]
        {
            println!("{}", bridge_ip);
        }
        // Register a new user.
        let username = bridge::register_user(bridge_ip, "huelib-rs example").unwrap();
        let bridge = Bridge::new(bridge_ip, username);
        let lights = bridge.get_all_lights().unwrap();
        #[cfg(debug_assertions)]
        {
            println!("{:?}", lights);
        }
    }
    Ok(json!({}))
}
