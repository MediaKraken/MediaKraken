// https://github.com/nn1ks/huelib-rs

use huelib::resource::Light;
use huelib::resource::{light, Adjust, Alert};
use huelib::Color;
use huelib::{bridge, Bridge};
use std::net::IpAddr;

pub async fn mk_hardware_phue_register_username(
    bridge_ip: IpAddr,
) -> Result<String, Box<dyn std::error::Error>> {
    let client_key = bridge::register_user(bridge_ip, "mediakraken").unwrap();
    Ok(client_key)
}

pub async fn mk_hardware_phue_bridge_discover() -> Result<Vec<IpAddr>, Box<dyn std::error::Error>> {
    let hub_ip_addresses = bridge::discover_nupnp().unwrap();
    Ok(hub_ip_addresses)
}

pub async fn mk_hardware_phue_bridge_discover_lights(
    bridge_ip: IpAddr,
    client_key: String,
) -> Result<Vec<Light>, Box<dyn std::error::Error>> {
    let bridge = Bridge::new(bridge_ip, client_key);
    let lights = bridge.get_all_lights().unwrap();
    Ok(lights)
}

pub async fn mk_hardware_phue_bridge_remove_light(
    bridge_ip: IpAddr,
    client_key: String,
    light_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let bridge = Bridge::new(bridge_ip, client_key);
    let _result = bridge.delete_light(light_id);
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
    let light_modifier = light::StateModifier::new()
        .with_on(true)
        .with_saturation(Adjust::Override(light_saturation.unwrap() as u8))
        .with_alert(Alert::Select)
        .with_brightness(Adjust::Decrement(light_brightness.unwrap() as u8));
    let _response = bridge.set_light_state(light_id, &light_modifier).unwrap();
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
    let _response = bridge.set_light_state(light_id, &light_modifier).unwrap();
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
    let _response = bridge.set_light_state(light_id, &light_modifier).unwrap();
    Ok(())
}
