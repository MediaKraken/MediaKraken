// https://www.gutenberg.org/

// feed of new books
// http://www.gutenberg.org/cache/epub/feeds/today.rss

use mk_lib_logging::mk_lib_logging;
use mk_lib_network::mk_lib_network;
use serde_json::json;
use std::error::Error;
use stdext::function_name;
