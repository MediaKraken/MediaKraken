#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/serialport/serialport-rs
// apt install pkg-config libudev-dev

use serialport::{available_ports, SerialPortType, DataBits, StopBits};
use std::io::{self, Write};
use std::time::Duration;
