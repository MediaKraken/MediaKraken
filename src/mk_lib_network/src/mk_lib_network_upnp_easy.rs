// https://github.com/FloGa/upnp-daemon/tree/develop/crates/easy-upnp

use cidr_utils::cidr::Ipv4Cidr;
use easy_upnp::{add_ports, delete_ports, PortMappingProtocol, UpnpConfig};
use std::error::Error;

// nmap -sU -p 1900 --script=upnp-info 192.168.1.1

fn get_configs() -> Result<[UpnpConfig; 3], Box<dyn Error>> {
    let config_no_address = UpnpConfig {
        address: None,
        port: 8900,
        protocol: PortMappingProtocol::TCP,
        duration: 3600,
        comment: "MediaKraken".to_string(),
    };

    let config_specific_address = UpnpConfig {
        address: Some(Ipv4Cidr::from_str("192.168.0.10/24")?),
        port: 8080,
        protocol: PortMappingProtocol::TCP,
        duration: 3600,
        comment: "Webserver alternative".to_string(),
    };

    let config_address_range = UpnpConfig {
        address: Some(Ipv4Cidr::from_str("192.168.0")?),
        port: 8081,
        protocol: PortMappingProtocol::TCP,
        duration: 3600,
        comment: "Webserver second alternative".to_string(),
    };

    Ok([
        config_no_address,
        config_specific_address,
        config_address_range,
    ])
}

pub async fn upnp_open_ports() -> Result<(), Box<dyn Error>> {
    add_ports(get_configs()?);
    Ok(())
}

pub async fn upnp_delete_ports() -> Result<(), Box<dyn Error>> {
    delete_ports(get_configs()?);
    Ok(())
}
