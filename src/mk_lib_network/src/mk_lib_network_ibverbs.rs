// https://github.com/jonhoo/rust-ibverbs

use serde_json::json;
use stdext::function_name;

pub async fn mk_lib_network_ibverbs_discover() {
    let ctx = ibverbs::devices()
        .unwrap()
        .iter()
        .next()
        .expect("no rdma device available")
        .open()
        .unwrap();
}
