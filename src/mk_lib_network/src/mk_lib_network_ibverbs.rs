// https://github.com/jonhoo/rust-ibverbs

use mk_lib_logging::mk_lib_logging;
use serde_json::json;
use stdext::function_name;

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
