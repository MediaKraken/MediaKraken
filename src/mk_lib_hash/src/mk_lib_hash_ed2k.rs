#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/runfalk/ed2k-rs

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use stdext::function_name;
use serde_json::json;

pub fn mk_file_hash_ed2k(file_to_read: &str) -> Result<String, Box<dyn Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        );
    }
    let ed2k = Ed2k::from_path(file_to_read)?;
    return ed2k;
}
