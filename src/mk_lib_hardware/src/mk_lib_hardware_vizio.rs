// https://github.com/ConnorTroy/smartcast

use serde_json::json;
use smartcast::Device;
use stdext::function_name;

pub async fn mk_hardware_vizio_discover() -> Result<(), smartcast::Error> {
    let ssdp_devices = smartcast::discover_devices().await?;
    let dev_by_ssdp = ssdp_devices[0].clone();
    let ip_addr = dev_by_ssdp.ip();
    let uuid = dev_by_ssdp.uuid();
    let _dev_by_ip = Device::from_ip(ip_addr).await?;
    let _dev_by_uuid = Device::from_uuid(uuid).await?;
    Ok(())
}
