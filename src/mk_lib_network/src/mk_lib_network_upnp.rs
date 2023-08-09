// https://github.com/jakobhellermann/rupnp

use rupnp::ssdp::SearchTarget;
use std::time::Duration;

// nmap -sU -p 1900 --script=upnp-info 192.168.1.1

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
