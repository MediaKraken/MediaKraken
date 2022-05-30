#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/serialport/serialport-rs

use serialport::{available_ports, SerialPortType, DataBits, StopBits};
use std::io::{self, Write};
use std::time::Duration;
