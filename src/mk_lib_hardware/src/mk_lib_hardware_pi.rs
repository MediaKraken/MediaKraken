use rascam::*;
use mk_lib_logging::mk_lib_logging;
use rppal::gpio::Gpio;
use serde_json::json;
use std::error::Error;
use stdext::function_name;
use tokio::time::{sleep, Duration};

// let gpio = Gpio::new()?;
// let i2c = I2c::new()?;
// let pwm = Pwm::new(Channel::Pwm0)?;
// let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 16_000_000, Mode::Mode0)?;
// let uart = Uart::new(115_200, Parity::None, 8, 1)?;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
//const GPIO_LED: u8 = 23;

pub async fn mk_lib_hardware_pi_led_flash(
    gpio_pin: u8,
    milliseconds: u64,
) -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let mut pin = Gpio::new()?.get(gpio_pin)?.into_output();
    loop {
        pin.toggle();
        sleep(Duration::from_millis(milliseconds)).await;
    }
}

pub async fn mk_lib_hardware_pi_take_image(image_file_name: String) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let info = info().unwrap();
    if info.cameras.len() > 0 {
        #[cfg(debug_assertions)]
        {
            mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({ "info": info })).await.unwrap();
        }
        simple_sync(&info.cameras[0], image_file_name);
    }
}

async fn simple_sync(info: &CameraInfo, image_file_name: String) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let mut camera = SimpleCamera::new(info.clone()).unwrap();
    camera.activate().unwrap();
    let b = camera.take_one().unwrap();
    File::create(image_file_name).unwrap().write_all(&b).unwrap();
}
