// https://github.com/serialport/serialport-rs
// apt install pkg-config libudev-dev
// serialport = "4.2.2"

use serialport::{DataBits, SerialPort, StopBits};
use std::io::Write;
use std::time::Duration;

pub async fn serial_port_discover() -> Result<(), Box<dyn std::error::Error>> {
    let ports = serialport::available_ports()?;
    for p in ports {
        println!("port: {}  type: {:?}", p.port_name, p.port_type);
    }
    Ok(())
}

pub async fn serial_port_open(
    serial_device: String,
    serial_speed: u32,
    serial_stop_bits: StopBits,
    serial_data_bits: DataBits,
) -> Result<Box<dyn SerialPort>, Box<dyn std::error::Error>> {
    // "/dev/ttyUSB0"
    let port = serialport::new(serial_device, serial_speed) // 115_200
        .stop_bits(serial_stop_bits)
        .data_bits(serial_data_bits)
        .timeout(Duration::from_millis(10))
        .open()?;
    Ok(port)
}

pub async fn serial_port_write(
    mut port: Box<dyn SerialPort>,
) -> Result<(), Box<dyn std::error::Error>> {
    let output = "This is a test. This is only a test.".as_bytes();
    port.write_all(output)?;
    Ok(())
}

pub async fn serial_port_read(
    mut port: Box<dyn SerialPort>,
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut serial_buf: Vec<u8> = vec![0; 32];
    let bytes_read = port.read(serial_buf.as_mut_slice())?;
    Ok(bytes_read)
}
