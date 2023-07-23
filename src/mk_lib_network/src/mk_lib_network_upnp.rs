// https://github.com/jakobhellermann/rupnp

use rupnp::ssdp::SearchTarget;
use serde_json::json;
use std::time::Duration;
use stdext::function_name;

pub async fn upnp_discover() -> Result<(), rupnp::Error> {
    let devices = rupnp::discover(&SearchTarget::RootDevice, Duration::from_secs(3)).await?;
    pin_utils::pin_mut!(devices);
    // while let Some(device) = devices.try_next().await? {
    //     #[cfg(debug_assertions)]
    //     {
    //         mk_lib_logging::mk_logging_post_elk(std::module_path!(),
    //         json!({ "type": device.device_type(), "name": device.friendly_name(), "url": device.url() })).await.unwrap();
    //     }
    // }
    Ok(())
}
