// https://github.com/nn1ks/huelib-rs

use mk_lib_logging::mk_lib_logging;

use huelib::resource::Light;
use huelib::resource::{light, Adjust, Alert};
use huelib::Color;
use huelib::{bridge, Bridge};
use serde_json::json;
use std::net::IpAddr;
use stdext::function_name;

pub async fn mk_hardware_phue_register_username(
    bridge_ip: IpAddr,
) -> Result<String, Box<dyn std::error::Error>> {
    let client_key = bridge::register_user(bridge_ip, "mediakraken").unwrap();
    Ok(client_key)
}

pub async fn mk_hardware_phue_bridge_discover() -> Result<Vec<IpAddr>, Box<dyn std::error::Error>> {
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
    light_saturation: u8,
    light_brightness: u8,
) -> Result<(), Box<dyn std::error::Error>> {
    let bridge = Bridge::new(bridge_ip, client_key);
    let light_modifier = light::StateModifier::new()
        .with_on(true)
        .with_saturation(Adjust::Override(light_saturation))
        .with_alert(Alert::Select)
        .with_brightness(Adjust::Decrement(light_brightness));
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
