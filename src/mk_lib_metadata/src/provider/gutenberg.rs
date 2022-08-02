#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://www.gutenberg.org/

// feed of new books
// http://www.gutenberg.org/cache/epub/feeds/today.rss

use std::error::Error;

#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;
