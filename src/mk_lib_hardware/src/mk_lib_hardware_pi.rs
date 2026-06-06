use rascam::*;
use rppal::gpio::Gpio;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use tokio::time::{Duration, sleep};

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
    let mut pin = Gpio::new()?.get(gpio_pin)?.into_output();
    loop {
        pin.toggle();
        sleep(Duration::from_millis(milliseconds)).await;
    }
}

pub async fn mk_lib_hardware_pi_take_image(image_file_name: String) -> Result<(), Box<dyn Error>> {
    let info = info()?;
    if info.cameras.is_empty() {
        return Err("no cameras detected".into());
    }
    let camera_info = info.cameras[0].clone();
    // rascam performs synchronous MMAL/camera I/O; keep it off the async
    // runtime so taking a photo doesn't stall other tasks on this worker.
    tokio::task::spawn_blocking(move || -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut camera = SimpleCamera::new(camera_info)?;
        camera.activate()?;
        let b = camera.take_one()?;
        let mut file = File::create(&image_file_name)?;
        file.write_all(&b)?;
        Ok(())
    })
    .await?
    .map_err(Into::into)
}
