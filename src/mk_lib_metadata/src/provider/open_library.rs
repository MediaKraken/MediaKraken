#![cfg_attr(debug_assertions, allow(dead_code))]

// https://openlibrary.org/

// https://openlibrary.org/data/ol_cdump_latest.txt.gz
// use above to preload book metadata

// covers?
// https://archive.org/details/amazon_2007_covers

// https://github.com/bspradling/open-library
// open-library = "0.7.1"

use open_library::models::books::BibliographyKey;
use open_library::models::books::BibliographyKey::{ISBN, LCCN, OCLC, OLID};
use open_library::OpenLibraryClient;
use serde_json::json;
use std::error::Error;
use stdext::function_name;

use crate::mk_lib_logging;

#[path = "../../mk_lib_network.rs"]
mod mk_lib_network;
