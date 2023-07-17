use mk_lib_hardware;

#[tokio::main]
async fn main() {
    mk_lib_hardware::mk_lib_hardware_onvif::mk_lib_hardware_onvif_discovery().await;
}
