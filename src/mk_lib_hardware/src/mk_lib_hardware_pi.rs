#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use rascam::*;
use rppal::gpio::Gpio;
use rppal::i2c::I2c;
use rppal::pwm::{Channel, Pwm};
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use rppal::uart::{Parity, Uart};
// [dependencies]
// rppal = { version = "0.12.0", features = ["hal"] }
// rascam = "0.0.2"
use std::error::Error;
use std::thread;
use tokio::time::{Duration, sleep};

// let gpio = Gpio::new()?;
// let i2c = I2c::new()?;
// let pwm = Pwm::new(Channel::Pwm0)?;
// let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 16_000_000, Mode::Mode0)?;
// let uart = Uart::new(115_200, Parity::None, 8, 1)?;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
//const GPIO_LED: u8 = 23;

pub async fn mk_lib_hardware_pi_led_flash(gpio_pin: u8, milliseconds: u32)
                                          -> Result<(), Box<dyn Error>> {
    let mut pin = Gpio::new()?.get(gpio_pin)?.into_output();
    loop {
        pin.toggle();
        sleep(Duration::from_milliseconds(milliseconds)).await;
    }
}

pub async fn mk_lib_hardware_pi_take_image(image_file_name: String) {
    let info = info().unwrap();
    if info.cameras.len() > 0 {
        #[cfg(debug_assertions)]
        {
        println!("{}", info);}
        simple_sync(&info.cameras[0], image_file_name);
    }
}

fn simple_sync(info: &CameraInfo, image_file_name String) {
    let mut camera = SimpleCamera::new(info.clone()).unwrap();
    camera.activate().unwrap();
    let b = camera.take_one().unwrap();
    File::create(image_file_name).unwrap().write_all(&b).unwrap();
}
