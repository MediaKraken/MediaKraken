use serde::Deserialize;
use std::error::Error;
use std::fmt::{Display, Formatter};
use tokio::process::Command;

const WINDOWS_ONLY_ERROR: &str = "TWAIN scanning is only supported on Windows hosts";

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct TwainDevice {
    pub id: String,
    pub name: String,
}

#[derive(Debug)]
pub enum TwainError {
    UnsupportedPlatform,
    CommandFailed(String),
    InvalidJson(serde_json::Error),
}

impl Display for TwainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TwainError::UnsupportedPlatform => write!(f, "{}", WINDOWS_ONLY_ERROR),
            TwainError::CommandFailed(message) => write!(f, "TWAIN command failed: {}", message),
            TwainError::InvalidJson(error) => {
                write!(f, "Unable to parse TWAIN command output: {}", error)
            }
        }
    }
}

impl Error for TwainError {}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum TwainDiscoveryPayload {
    Empty,
    Single(TwainDevice),
    Multiple(Vec<TwainDevice>),
}

fn parse_twain_devices(payload: &str) -> Result<Vec<TwainDevice>, TwainError> {
    if payload.trim().is_empty() {
        return Ok(Vec::new());
    }

    let parsed =
        serde_json::from_str::<TwainDiscoveryPayload>(payload).map_err(TwainError::InvalidJson)?;

    match parsed {
        TwainDiscoveryPayload::Empty => Ok(Vec::new()),
        TwainDiscoveryPayload::Single(device) => Ok(vec![device]),
        TwainDiscoveryPayload::Multiple(devices) => Ok(devices),
    }
}

fn quote_for_powershell(input: &str) -> String {
    input.replace('\'', "''")
}

pub async fn mk_lib_hardware_twain_discover() -> Result<Vec<TwainDevice>, TwainError> {
    if !cfg!(target_os = "windows") {
        return Err(TwainError::UnsupportedPlatform);
    }

    let discover_script = r#"
$deviceManager = New-Object -ComObject WIA.DeviceManager
$devices = @($deviceManager.DeviceInfos | ForEach-Object {
    [PSCustomObject]@{
        id = $_.DeviceID
        name = $_.Properties['Name'].Value
    }
})
$devices | ConvertTo-Json -Compress
"#;

    let output = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-Command")
        .arg(discover_script)
        .output()
        .await
        .map_err(|error| TwainError::CommandFailed(error.to_string()))?;

    if !output.status.success() {
        return Err(TwainError::CommandFailed(
            String::from_utf8_lossy(&output.stderr).trim().to_string(),
        ));
    }

    parse_twain_devices(&String::from_utf8_lossy(&output.stdout))
}

pub async fn mk_lib_hardware_twain_scan_to_file(
    device_id: &str,
    output_file: &str,
) -> Result<(), TwainError> {
    if !cfg!(target_os = "windows") {
        return Err(TwainError::UnsupportedPlatform);
    }

    let escaped_device_id = quote_for_powershell(device_id);
    let escaped_output_file = quote_for_powershell(output_file);
    let scan_script = format!(
        r#"
$deviceManager = New-Object -ComObject WIA.DeviceManager
$deviceInfo = $deviceManager.DeviceInfos | Where-Object {{ $_.DeviceID -eq '{device_id}' }} | Select-Object -First 1
if (-not $deviceInfo) {{
    throw 'Unable to find TWAIN device: {device_id}'
}}

$device = $deviceInfo.Connect()
$item = $device.Items.Item(1)
$image = $item.Transfer('{{B96B3CAF-0728-11D3-9D7B-0000F81EF32E}}')
$image.SaveFile('{output_file}')
"#,
        device_id = escaped_device_id,
        output_file = escaped_output_file,
    );

    let output = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-Command")
        .arg(scan_script)
        .output()
        .await
        .map_err(|error| TwainError::CommandFailed(error.to_string()))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(TwainError::CommandFailed(
            String::from_utf8_lossy(&output.stderr).trim().to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{TwainDevice, parse_twain_devices};

    #[test]
    fn parse_twain_devices_array_payload() {
        let devices = parse_twain_devices(
            r#"[{"id":"scanner-1","name":"Flatbed"},{"id":"scanner-2","name":"ADF"}]"#,
        )
        .expect("parser should handle array payload");

        assert_eq!(
            devices,
            vec![
                TwainDevice {
                    id: String::from("scanner-1"),
                    name: String::from("Flatbed"),
                },
                TwainDevice {
                    id: String::from("scanner-2"),
                    name: String::from("ADF"),
                },
            ]
        );
    }

    #[test]
    fn parse_twain_devices_single_payload() {
        let devices = parse_twain_devices(r#"{"id":"scanner-1","name":"Flatbed"}"#)
            .expect("parser should handle single object payload");

        assert_eq!(
            devices,
            vec![TwainDevice {
                id: String::from("scanner-1"),
                name: String::from("Flatbed"),
            }]
        );
    }
}
