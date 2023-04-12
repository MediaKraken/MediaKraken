#![cfg_attr(debug_assertions, allow(dead_code))]

// https://github.com/maxjoehnk/soundcloud-rs
// soundcloud = "0.4"

use serde_json::json;
use soundcloud::Client;
use stdext::function_name;

use crate::mk_lib_logging;
