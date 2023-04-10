#![cfg_attr(debug_assertions, allow(dead_code))]

// http://dev-guide.pitchfork.com/docs.html

use serde_json::json;
use std::error::Error;
use stdext::function_name;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;
