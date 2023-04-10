#![cfg_attr(debug_assertions, allow(dead_code))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

// https://github.com/ali-raheem/LGRemote.rs
// LGremote = "0.2.1"

use serde_json::json;
use stdext::function_name;
use LGremote::{COMMAND_CODES, LGTV};

pub async fn mk_lib_hardware_lg_connect_key() {}

pub async fn mk_lib_hardware_lg_send_command() {}
