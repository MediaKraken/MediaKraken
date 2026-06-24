// https://github.com/dariusc93/rust-igd

use igd_next as igd;
use std::error::Error;
use std::net::{IpAddr, SocketAddr};

pub async fn upnp_discover_gateway() -> Result<(String, String), Box<dyn Error>> {
    let gateway = igd::search_gateway(Default::default())
        .map_err(|err| format!("Error: {err}"))?;
    let ext_addr = gateway.get_external_ip()
        .map_err(|err| format!("There was an error! {err}"))?;
    println!(
        "Local gateway: {}, External ip address: {}",
        gateway.addr, ext_addr
    );
    Ok((gateway.addr.to_string(), ext_addr.to_string()))
}

pub async fn upnp_add_port(
    local_addr: String,
    internal_port: u16,
    external_port: u16,
) -> Result<(), Box<dyn Error>> {
    let gateway = igd::search_gateway(Default::default())
        .map_err(|err| format!("Error: {err}"))?;
    let local_addr = local_addr.parse::<IpAddr>().map_err(|e| format!("invalid ip address: {e}"))?;
    let local_addr = SocketAddr::new(local_addr, internal_port);
    gateway.add_port(
        igd::PortMappingProtocol::TCP,
        external_port,
        local_addr,
        60,
        "MediaKraken",
    ).map_err(|err| format!("There was an error! {err}"))?;
    Ok(())
}

pub async fn upnp_delete_port(external_port: u16) -> Result<(), Box<dyn Error>> {
    let gateway = igd::search_gateway(Default::default())
        .map_err(|err| format!("Error: {err}"))?;
    gateway.remove_port(igd::PortMappingProtocol::TCP, external_port)
        .map_err(|err| format!("There was an error! {err}"))?;
    Ok(())
}
