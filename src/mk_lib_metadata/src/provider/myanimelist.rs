#![cfg_attr(debug_assertions, allow(dead_code))]

// https://github.com/AnActualEmerald/lib-mal
// lib-mal = "0.5.1"

use lib_mal::{MALClient, MALError};
use serde_json::json;
use stdext::function_name;

use crate::mk_lib_logging;
