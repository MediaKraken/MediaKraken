use mk_lib_hardware;

#[tokio::main]
async fn main() {
    let results = mk_lib_hardware::mk_lib_hardware_roku::mk_lib_hardware_roku_discover().await;
    println!("Result: {:?}", results);
}
