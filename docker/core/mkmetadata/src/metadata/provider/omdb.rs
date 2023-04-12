#![cfg_attr(debug_assertions, allow(dead_code))]

// https://www.omdbapi.com/
// https://github.com/aldrio/omdb-rs
// omdb = "0.3.2"

use serde_json::json;
use std::error::Error;
use stdext::function_name;

use crate::mk_lib_logging;

use crate::mk_lib_network;
