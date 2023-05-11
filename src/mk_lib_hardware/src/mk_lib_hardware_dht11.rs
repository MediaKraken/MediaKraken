// https://github.com/Xavientois/rppal-dht11-rs

use mk_lib_logging::mk_lib_logging;
use rppal::{
    gpio::{Gpio, Mode},
    hal::Delay,
};
use rppal_dht11::{Dht11, Measurement};
use serde_json::json;
use stdext::function_name;

const DHT11_PIN: u8 = 17;

pub async fn mk_lib_hardware_dht11_get_reading() {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let pin = Gpio::new()
        .unwrap()
        .get(DHT11_PIN)
        .unwrap()
        .into_io(Mode::Output);
    let mut dht11 = Dht11::new(pin);
    let mut delay = Delay::new();
    loop {
        match dht11.perform_measurement_with_retries(&mut delay, 20) {
            Ok(Measurement {
                temperature,
                humidity,
            }) => {
                let (temperature, humidity) = (temperature as f64 / 10.0, humidity as f64 / 10.0);
                #[cfg(debug_assertions)]
                {
                    mk_lib_logging::mk_logging_post_elk(
                        std::module_path!(),
                        json!({ "Temp": temperature, "Hum": humidity }),
                    )
                    .await
                    .unwrap();
                }
            }
            Err(e) => eprintln!("Failed to perform measurement: {e:?}"),
        }
    }
}
