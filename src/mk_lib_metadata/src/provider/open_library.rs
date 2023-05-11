// https://openlibrary.org/

// https://openlibrary.org/data/ol_cdump_latest.txt.gz
// use above to preload book metadata

// covers?
// https://archive.org/details/amazon_2007_covers

// https://github.com/bspradling/open-library

use mk_lib_logging::mk_lib_logging;
use mk_lib_network::mk_lib_network;
use open_library::models::books::BibliographyKey;
use open_library::models::books::BibliographyKey::{ISBN, LCCN, OCLC, OLID};
use open_library::OpenLibraryClient;
use serde_json::json;
use std::error::Error;
use stdext::function_name;
