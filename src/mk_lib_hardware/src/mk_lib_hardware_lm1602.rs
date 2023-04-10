#![cfg_attr(debug_assertions, allow(dead_code))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use serde_json::json;
use stdext::function_name;

// https://github.com/KuabeM/lcd-lcm1602-i2c
