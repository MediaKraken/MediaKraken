use futures_util::{pin_mut, stream::StreamExt};
use std::time::Duration;

const FIRETV_SERVICE_NAMES: [&str; 2] = [
    "_adb-tls-connect._tcp.local",
    "_androidtvremote2._tcp.local",
];

pub async fn mk_lib_hardware_firetv_discover() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut devices = Vec::new();

    for service_name in FIRETV_SERVICE_NAMES {
        let stream = mdns::discover::all(service_name, Duration::from_secs(5))?.listen();
        pin_mut!(stream);

        while let Some(Ok(response)) = stream.next().await {
            if let Some(hostname) = response.hostname() {
                let hostname = hostname.to_string();
                if !devices.iter().any(|existing| existing == &hostname) {
                    devices.push(hostname);
                }
            }
        }
    }

    Ok(devices)
}
