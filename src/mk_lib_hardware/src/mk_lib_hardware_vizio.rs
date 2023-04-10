#![cfg_attr(debug_assertions, allow(dead_code))]

// https://github.com/ConnorTroy/smartcast
// smartcast = "0.1.1"

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use smartcast::Device;
use stdext::function_name;
use serde_json::json;

put async fn mk_hardware_vizio_discover() {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let ssdp_devices = smartcast::discover_devices().await?;
    let dev_by_ssdp = ssdp_devices[0].clone();
    let ip_addr = dev_by_ssdp.ip();
    let uuid = dev_by_ssdp.uuid();
    let dev_by_ip = Device::from_ip(ip_addr).await?;
    let dev_by_uuid = Device::from_uuid(uuid).await?;
}
