#![cfg_attr(debug_assertions, allow(dead_code))]

// https://github.com/nn1ks/huelib-rs
// huelib = "0.13.2"

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use huelib::resource::sensor;
use huelib::{bridge, Bridge};
use serde_json::json;
use serde_json::json;
use stdext::function_name;

pub async fn mk_hardware_phue_discover() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let hub_ip_addresses = bridge::discover_nupnp().unwrap();
    for bridge_ip in hub_ip_addresses {
        #[cfg(debug_assertions)]
        {
            mk_lib_logging::mk_logging_post_elk(
                std::module_path!(),
                json!({ "bridge_ip": bridge_ip }),
            )
            .await
            .unwrap();
        }
        // Register a new user.
        let username = bridge::register_user(bridge_ip, "huelib-rs example").unwrap();
        let bridge = Bridge::new(bridge_ip, username);
        let lights = bridge.get_all_lights().unwrap();
        #[cfg(debug_assertions)]
        {
            mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({ "bridge": lights }))
                .await
                .unwrap();
        }
    }
    Ok(json!({}))
}
