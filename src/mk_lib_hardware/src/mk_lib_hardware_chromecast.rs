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

#[cfg(test)]
mod tests {
    use super::ChromecastDevice;
    use serde_json;

    #[test]
    fn chromecast_device_serialize() {
        let device = ChromecastDevice {
            hostname: "living-room.local".to_string(),
            address: "192.168.1.50:8008".to_string(),
        };
        let json_value = serde_json::to_value(&device).unwrap();
        assert_eq!(json_value["hostname"], "living-room.local");
        assert_eq!(json_value["address"], "192.168.1.50:8008");
    }

    #[test]
    fn chromecast_device_clone() {
        let device = ChromecastDevice {
            hostname: "bedroom.local".to_string(),
            address: "192.168.1.51:8008".to_string(),
        };
        let cloned = device.clone();
        assert_eq!(device.hostname, cloned.hostname);
        assert_eq!(device.address, cloned.address);
    }

    #[test]
    fn chromecast_device_debug_format() {
        let device = ChromecastDevice {
            hostname: "kitchen.local".to_string(),
            address: "192.168.1.52:8008".to_string(),
        };
        let debug_str = format!("{:?}", device);
        assert!(debug_str.contains("ChromecastDevice"));
    }
}

pub async fn mk_hardware_chromecast_discover()
-> Result<Vec<ChromecastDevice>, Box<dyn std::error::Error>> {
    let stream = mdns::discover::all(CHROMECAST_SERVICE_NAME, Duration::from_secs(15))?.listen();

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
