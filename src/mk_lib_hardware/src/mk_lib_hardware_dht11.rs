#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/Xavientois/rppal-dht11-rs
// rppal-dht11 = "0.4.0"

use rppal::{
    gpio::{Gpio, Mode},
    hal::Delay,
};
use rppal_dht11::{Dht11, Measurement};

const DHT11_PIN: u8 = 17;

pub async fn mk_lib_hardware_dht11_get_reading() {
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
                    println!("Temp: {temperature:.1}C Hum: {humidity:.1}");
                }
            }
            Err(e) => eprintln!("Failed to perform measurement: {e:?}"),
        }
    }
}
