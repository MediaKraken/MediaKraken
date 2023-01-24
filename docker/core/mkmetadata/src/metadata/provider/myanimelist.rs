#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/AnActualEmerald/lib-mal
// lib-mal = "0.5.1"

use lib_mal::{MALClient, MALError};
use stdext::function_name;
use serde_json::json;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;
