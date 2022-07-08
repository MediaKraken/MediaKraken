#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/serialport/serialport-rs
// apt install pkg-config libudev-dev

use serialport::{available_ports, DataBits, SerialPortType, StopBits};
use std::io::{self, Write};
use std::time::Duration;

pub async fn serial_port_discover() -> Result<(), std::Error> {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{} {}", p.port_name, p.port_type);
    }
    Ok(())
}

pub async fn serial_port_open(
    serial_device: String,
    serial_speed: i8,
    serial_stop_bits: StopBits,
    serial_data_bits: DataBits,
) -> Result<(serialport), std::Error> {
    // "/dev/ttyUSB0"
    let port = serialport::new(serial_device, serial_speed) // 115_200
        .stop_bits(serial_stop_bits)
        .data_bits(serial_data_bits)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");
    Ok(port)
}

pub async fn serial_port_write() -> Result<(), std::Error> {
    let output = "This is a test. This is only a test.".as_bytes();
    port.write(output).expect("Write failed!");
}

pub async fn serial_port_read() -> Result<(), std::Error> {
    let mut serial_buf: Vec<u8> = vec![0; 32];
    port.read(serial_buf.as_mut_slice())
        .expect("Found no data!");
}
