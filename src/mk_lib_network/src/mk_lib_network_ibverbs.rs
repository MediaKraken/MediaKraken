#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

// https://github.com/jonhoo/rust-ibverbs
// ibverbs = "0.7.0"

pub async fn mk_lib_network_ibverbs_discover() {
    let ctx = ibverbs::devices()
        .unwrap()
        .iter()
        .next()
        .expect("no rdma device available")
        .open()
        .unwrap();
}
