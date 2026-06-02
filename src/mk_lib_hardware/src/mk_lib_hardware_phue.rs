// https://github.com/nn1ks/huelib-rs

use huelib::resource::Light;
use huelib::resource::{light, Adjust, Alert};
use huelib::Color;
use huelib::{bridge, Bridge};
use std::net::IpAddr;

/// Register a device name on the Philips Hue bridge and return the
/// bridge-assigned client key.
///
/// `device_name` is shown in the Hue app's "connected apps" list and
/// is used purely for identification; pick a short label identifying
/// the installation (e.g. a hostname). The bridge's link button must
/// be pressed before calling this.
pub async fn mk_hardware_phue_register_username(
    bridge_ip: IpAddr,
    device_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client_key = bridge::register_user(bridge_ip, device_name)?;
    Ok(client_key)
}

pub async fn mk_hardware_phue_bridge_discover() -> Result<Vec<IpAddr>, Box<dyn std::error::Error>> {
    let hub_ip_addresses = bridge::discover_nupnp()?;
    Ok(hub_ip_addresses)
}

pub async fn mk_hardware_phue_bridge_discover_lights(
    bridge_ip: IpAddr,
    client_key: String,
) -> Result<Vec<Light>, Box<dyn std::error::Error>> {
    let bridge = Bridge::new(bridge_ip, client_key);
    let lights = bridge.get_all_lights()?;
    Ok(lights)
}

pub async fn mk_hardware_phue_bridge_remove_light(
    bridge_ip: IpAddr,
    client_key: String,
    light_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let bridge = Bridge::new(bridge_ip, client_key);
    bridge.delete_light(light_id)?;
    Ok(())
}

pub async fn mk_hardware_phue_bridge_set_light(
    bridge_ip: IpAddr,
    client_key: String,
    light_id: String,
    light_saturation: Option<u64>,
    light_brightness: Option<u64>,
) -> Result<(), Box<dyn std::error::Error>> {
    let bridge = Bridge::new(bridge_ip, client_key);
    let mut modifier = light::StateModifier::new()
        .with_on(true)
        .with_alert(Alert::Select);
    if let Some(sat) = light_saturation {
        modifier = modifier.with_saturation(Adjust::Override(sat.min(u8::MAX as u64) as u8));
    }
    if let Some(bri) = light_brightness {
        modifier = modifier.with_brightness(Adjust::Override(bri.min(u8::MAX as u64) as u8));
    }
    bridge.set_light_state(light_id, &modifier)?;
    Ok(())
}

pub async fn mk_hardware_phue_bridge_set_color(
    bridge_ip: IpAddr,
    client_key: String,
    light_id: String,
    light_color: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let bridge = Bridge::new(bridge_ip, client_key);
    let light_modifier = light::StateModifier::new()
        .with_on(true)
        .with_color(Color::from_hex(light_color)?)
        .with_alert(Alert::Select);
    bridge.set_light_state(light_id, &light_modifier)?;
    Ok(())
}

pub async fn mk_hardware_phue_bridge_set_light_onoff(
    bridge_ip: IpAddr,
    client_key: String,
    light_id: String,
    light_on: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let bridge = Bridge::new(bridge_ip, client_key);
    let light_modifier = light::StateModifier::new().with_on(light_on);
    bridge.set_light_state(light_id, &light_modifier)?;
    Ok(())
}
