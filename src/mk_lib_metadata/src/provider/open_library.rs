#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://openlibrary.org/

// https://openlibrary.org/data/ol_cdump_latest.txt.gz
// use above to preload book metadata

// covers?
// https://archive.org/details/amazon_2007_covers

use std::error::Error;

#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;

