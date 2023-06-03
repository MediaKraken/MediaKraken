// https://github.com/Rahix/avr-hal

use panic_halt as _;

pub async fn mk_lib_hardware_arduino_blink(pins: pinmsthis, blink_delay: u16, blink_times: u64) {
    let mut led = pins.d13.into_output().downgrade();
    for _ in 0..blink_times {
        led.toggle();
        arduino_hal::delay_ms(blink_delay);
    }
}

pub async fn mk_lib_hardware_arduino_get_pins() {
    let device_pins = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(device_pins);
    Ok(pins)
}
