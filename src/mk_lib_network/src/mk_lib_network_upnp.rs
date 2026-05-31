// https://github.com/dariusc93/rust-igd

use igd_next as igd;
use std::error::Error;
use std::net::{IpAddr, SocketAddr};

pub async fn upnp_discover_gateway() -> Result<(), Box<dyn Error>> {
    match igd::search_gateway(Default::default()) {
        Err(ref err) => println!("Error: {err}"),
        Ok(gateway) => match gateway.get_external_ip() {
            Err(ref err) => {
                println!("There was an error! {err}");
            }
            Ok(ext_addr) => {
                println!(
                    "Local gateway: {}, External ip address: {}",
                    gateway.addr, ext_addr
                );
            }
        },
    }
    Ok(())
}

pub async fn upnp_add_port(
    local_addr: String,
    internal_port: u16,
    external_port: u16,
) -> Result<(), Box<dyn Error>> {
    match igd::search_gateway(Default::default()) {
        Err(ref err) => println!("Error: {err}"),
        Ok(gateway) => {
            let local_addr = local_addr.parse::<IpAddr>().map_err(|e| format!("invalid ip address: {e}"))?;
            let local_addr = SocketAddr::new(local_addr, internal_port);
            if let Err(ref err) = gateway.add_port(
                igd::PortMappingProtocol::TCP,
                external_port,
                local_addr,
                60,
                "MediaKraken",
            ) {
                println!("There was an error! {err}");
            }
        }
    }
    Ok(())
}

pub async fn upnp_delete_port(external_port: u16) -> Result<(), Box<dyn Error>> {
    match igd::search_gateway(Default::default()) {
        Err(ref err) => println!("Error: {err}"),
        Ok(gateway) => if let Err(ref err) = gateway.remove_port(igd::PortMappingProtocol::TCP, external_port) {
            println!("There was an error! {err}");
        },
    }
    Ok(())
}
