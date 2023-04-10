#![cfg_attr(debug_assertions, allow(dead_code))]

// https://github.com/jonhoo/rust-ibverbs
// ibverbs = "0.7.0"

use serde_json::json;
use stdext::function_name;

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

pub async fn mk_lib_network_ibverbs_discover() {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    let ctx = ibverbs::devices()
        .unwrap()
        .iter()
        .next()
        .expect("no rdma device available")
        .open()
        .unwrap();
}
