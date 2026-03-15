use futures_util::{pin_mut, stream::StreamExt};
use serde::Serialize;
use std::collections::HashSet;
use std::time::Duration;

const CHROMECAST_SERVICE_NAME: &str = "_googlecast._tcp.local";

#[derive(Debug, Clone, Serialize)]
pub struct ChromecastDevice {
    pub hostname: String,
    pub address: String,
}

pub async fn mk_hardware_chromecast_discover(
) -> Result<Vec<ChromecastDevice>, Box<dyn std::error::Error>> {
    let stream =
        mdns::discover::all(CHROMECAST_SERVICE_NAME, Duration::from_secs(15))?.listen();

    pin_mut!(stream);

    let mut seen = HashSet::new();
    let mut devices = Vec::new();

    while let Some(Ok(response)) = stream.next().await {
        let Some(hostname) = response.hostname() else {
            continue;
        };

        let Some(addr) = response.socket_address() else {
            continue;
        };

        let hostname = hostname.to_string();
        let address = addr.to_string();

        if seen.insert((hostname.clone(), address.clone())) {
            devices.push(ChromecastDevice { hostname, address });
        }
    }

    Ok(devices)
}