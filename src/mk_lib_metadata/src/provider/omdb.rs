#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://www.omdbapi.com/
// https://github.com/aldrio/omdb-rs
// omdb = "0.3.2"

use std::error::Error;

#[path = "../../mk_lib_logging.rs"]
mod mk_lib_logging;

#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;
