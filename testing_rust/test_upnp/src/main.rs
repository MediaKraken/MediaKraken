use mk_lib_network;

#[tokio::main]
async fn main() {
    let _result = mk_lib_network::mk_lib_network_upnp::upnp_discover_gateway().await;
}
