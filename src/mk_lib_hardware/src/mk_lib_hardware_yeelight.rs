// https://github.com/teppah/yeelib_rs

use std::time::Duration;
use yeelib_rs::{Light, YeeClient};

pub async fn mk_hardware_yeelight_brightness() {}

pub async fn mk_hardware_yeelight_discover() -> Result<Vec<Light>, Box<dyn std::error::Error>> {
    let client = YeeClient::new()?;
    // Yeelight discovery is advertised as best-effort; return an empty
    // list rather than looping forever so callers can decide whether to
    // retry or surface "no lights found" to the user.
    Ok(client.find_lights(Duration::from_secs(1)))
}

pub async fn mk_hardware_yeelight_power() {}

pub async fn mk_hardware_yeelight_rgb() {}

/*
let client = YeeClient::new()?;
  let mut lights: Vec<Light> = client.find_lights(Duration::from_secs(1));

  for light in lights.iter_mut() {
      light.set_power(PowerStatus::On, Transition::sudden())?;

      light.set_bright(50, Transition::sudden())?;

      light.set_ct_abx(3500,
                       Transition::smooth(Duration::from_millis(400))
                           .unwrap())?;

      light.toggle()?;
  } */
