// https://github.com/teppah/yeelib_rs

use mk_lib_logging::mk_lib_logging;
use serde_json::json;
use stdext::function_name;
use std::time::Duration;
use yeelib_rs::fields::{PowerStatus, Transition};
use yeelib_rs::{Light, YeeClient, YeeError};

pub async fn mk_hardware_yeelight_brightness() {}

pub async fn mk_hardware_yeelight_discover() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let client = YeeClient::new()?;
    let mut res: Vec<Light> = loop {
        let lights = client.find_lights(Duration::from_secs(1));
        // sometimes, it doesn't find anything, so rerun
        if lights.len() == 0 {
            #[cfg(debug_assertions)]
            {
                mk_lib_logging::mk_logging_post_elk(
                    std::module_path!(),
                    json!({ "YeeClient": "zero" }),
                )
                .await
                .unwrap();
            }
        } else {
            break lights;
        }
    };
    let light = res.get_mut(0).unwrap();
    // #[cfg(debug_assertions)]
    // {
    //     mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({ "light": light }))
    //         .await
    //         .unwrap();
    // }
    Ok(())
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
