#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/maxjoehnk/soundcloud-rs
// soundcloud = "0.4"

use soundcloud::Client;
use stdext::function_name;
use serde_json::json;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;
