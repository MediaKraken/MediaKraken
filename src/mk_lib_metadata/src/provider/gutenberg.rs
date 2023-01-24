#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://www.gutenberg.org/

// feed of new books
// http://www.gutenberg.org/cache/epub/feeds/today.rss

use std::error::Error;
use stdext::function_name;
use serde_json::json;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;
