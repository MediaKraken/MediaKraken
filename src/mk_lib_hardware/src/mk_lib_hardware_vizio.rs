// https://github.com/ConnorTroy/smartcast

use smartcast::Device;

pub async fn mk_hardware_vizio_discover() -> Result<(), smartcast::Error> {
    let Some(dev_by_ssdp) = smartcast::discover_devices().await?.into_iter().next() else {
        return Ok(());
    };
    let ip_addr = dev_by_ssdp.ip();
    let uuid = dev_by_ssdp.uuid();
    let _dev_by_ip = Device::from_ip(ip_addr).await?;
    let _dev_by_uuid = Device::from_uuid(uuid).await?;
    Ok(())
}
