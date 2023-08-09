use std::error::Error;

use mk_lib_network;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let vec_data = mk_lib_network::mk_lib_network_share::mk_network_share_scan_port("192.168.1".to_string()).await.unwrap();
    // let vec_data = mk_lib_network::mk_lib_network_nmap::mk_network_share_scan("192.168.1".to_string()).await.unwrap();
    println!("Data: {:?}", vec_data);
    Ok(())
}
