#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// http://dev-guide.pitchfork.com/docs.html

use std::error::Error;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;
