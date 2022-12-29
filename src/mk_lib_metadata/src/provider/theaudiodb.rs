#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://www.theaudiodb.com/api_guide.php

use std::error::Error;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;
