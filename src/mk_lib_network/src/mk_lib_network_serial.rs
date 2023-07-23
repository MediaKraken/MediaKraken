// https://github.com/serialport/serialport-rs
// apt install pkg-config libudev-dev

// TODO port this to https://github.com/berkowski/tokio-serial
use serde_json::json;
use serialport::{available_ports, DataBits, SerialPort, SerialPortType, StopBits};
use std::io::{self, Write};
use std::time::Duration;
use stdext::function_name;

pub async fn serial_port_discover() -> Result<(), Box<dyn std::error::Error>> {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        #[cfg(debug_assertions)]
        {
            // mk_lib_logging::mk_logging_post_elk(
            //     std::module_path!(),
            //     json!({ "port": p.port_name }), //, "type": p.port_type }),
            // )
            // .await
            // .unwrap();
        }
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
        .open()
        .expect("Failed to open port");
    Ok(port)
}

pub async fn serial_port_write(mut port: Box<dyn SerialPort>) -> Result<(), Box<dyn std::error::Error>> {
    let output = "This is a test. This is only a test.".as_bytes();
    port.write(output).expect("Write failed!");
    Ok(())
}

pub async fn serial_port_read(mut port:Box<dyn SerialPort>) -> Result<(), Box<dyn std::error::Error>> {
    let mut serial_buf: Vec<u8> = vec![0; 32];
    port.read(serial_buf.as_mut_slice())
        .expect("Found no data!");
    Ok(())
}
