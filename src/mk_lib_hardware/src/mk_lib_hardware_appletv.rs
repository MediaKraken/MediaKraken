use futures_util::{pin_mut, stream::StreamExt};
use std::time::Duration;

const APPLETV_SERVICE_NAME: &str = "_appletv-v2._tcp.local";

pub async fn mk_lib_hardware_appletv_discover() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let stream = mdns::discover::all(APPLETV_SERVICE_NAME, Duration::from_secs(5))?.listen();
    pin_mut!(stream);

    let mut devices = Vec::new();
    while let Some(Ok(response)) = stream.next().await {
        if let Some(hostname) = response.hostname() {
            let hostname = hostname.to_string();
            if !devices.iter().any(|existing| existing == &hostname) {
                devices.push(hostname);
            }
        }
    }

    Ok(devices)
}
