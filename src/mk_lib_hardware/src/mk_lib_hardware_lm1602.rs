#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use stdext::function_name;
use serde_json::json;

// https://github.com/KuabeM/lcd-lcm1602-i2c
