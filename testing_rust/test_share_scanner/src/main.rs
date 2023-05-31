use std::error::Error;

mod mk_lib_network_nmap;

mod mk_lib_file;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    mk_lib_network_nmap::mk_network_share_scan("192.168.1".to_string()).await.unwrap();
    Ok(())
}
