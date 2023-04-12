#![cfg_attr(debug_assertions, allow(dead_code))]

// https://www.theaudiodb.com/api_guide.php

use serde_json::json;
use std::error::Error;
use stdext::function_name;

use crate::mk_lib_logging;

#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;
