// https://github.com/guanicoe/LiquidCrystal_I2C-rs

use rppal::{gpio::Gpio, i2c::I2c};

//static LCD_ADDRESS: u8 = 0x27;

async fn mk_lib_hardware_i2c_lcd(lcd_address: u8, lcd_text: String) {
    let mut i2c = I2c::new().unwrap();
    let mut delay = rppal::hal::Delay;
    let mut lcd = screen::Lcd::new(&mut i2c, lcd_address, &mut delay).unwrap();
    lcd.set_display(screen::Display::On).unwrap();
    lcd.set_backlight(screen::Backlight::On).unwrap();
    lcd.print(lcd_text).unwrap();
}
