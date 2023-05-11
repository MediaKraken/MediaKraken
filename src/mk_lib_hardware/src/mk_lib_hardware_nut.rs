// https://github.com/aramperes/nut-rs
// https://crates.io/crates/rups

use mk_lib_logging::mk_lib_logging;
use serde_json::json;
use stdext::function_name;
use rups::blocking::Connection;
use rups::{Auth, ConfigBuilder};
