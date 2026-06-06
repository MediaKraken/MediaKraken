// https://github.com/aQaTL/sane-scan/tree/main
// apt install libsane libsane-dev

use std::ffi::OsStr;
use std::path::Path;
use std::process::Stdio;

use tokio::fs;
use tokio::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaneScannerDevice {
    pub id: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaneScanOutput {
    pub bytes_written: u64,
}

/// Discover scanners available via SANE using `scanimage -L`.
pub async fn mk_lib_hardware_sane_discover() -> Result<Vec<SaneScannerDevice>, String> {
    let output = Command::new("scanimage")
        .arg("-L")
        .output()
        .await
        .map_err(|error| format!("failed to run scanimage -L: {error}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let message = if stderr.is_empty() {
            "scanimage -L failed without stderr output".to_string()
        } else {
            format!("scanimage -L failed: {stderr}")
        };
        return Err(message);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(parse_scanimage_list(&stdout))
}

/// Scan from a selected SANE device and write the output file exactly where requested.
///
/// `format` is passed directly as `--format <format>` to `scanimage`.
pub async fn mk_lib_hardware_sane_scan<P: AsRef<Path>, F: AsRef<OsStr>>(
    device: &str,
    output_path: P,
    format: F,
) -> Result<SaneScanOutput, String> {
    let output_path = output_path.as_ref();

    if device.trim().is_empty() {
        return Err("scanner device id cannot be empty".to_string());
    }

    let scan_output = Command::new("scanimage")
        .arg("--device-name")
        .arg(device)
        .arg("--format")
        .arg(format)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|error| format!("failed to run scanimage: {error}"))?;

    if !scan_output.status.success() {
        let stderr = String::from_utf8_lossy(&scan_output.stderr)
            .trim()
            .to_string();
        let message = if stderr.is_empty() {
            "scanimage failed without stderr output".to_string()
        } else {
            format!("scanimage failed: {stderr}")
        };
        return Err(message);
    }

    fs::write(output_path, &scan_output.stdout)
        .await
        .map_err(|error| {
            format!(
                "failed to write scan output to {}: {error}",
                output_path.display()
            )
        })?;

    Ok(SaneScanOutput {
        bytes_written: scan_output.stdout.len() as u64,
    })
}

fn parse_scanimage_list(stdout: &str) -> Vec<SaneScannerDevice> {
    stdout
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if !line.starts_with("device `") {
                return None;
            }

            let id_start = "device `".len();
            let id_end = line[id_start..].find('\'')? + id_start;
            let id = line[id_start..id_end].trim();
            if id.is_empty() {
                return None;
            }

            let description_start =
                line[id_end + 1..].find(" is a ")? + id_end + 1 + " is a ".len();
            let description = line[description_start..]
                .trim()
                .trim_matches('\'')
                .trim_matches('`')
                .to_string();

            Some(SaneScannerDevice {
                id: id.to_string(),
                description,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{parse_scanimage_list, SaneScanOutput, SaneScannerDevice};

    #[test]
    fn parses_scanimage_output_lines() {
        let input = "device `escl:Brother DCP-L2540DW series [B1234]' is a Brother scanner\n\
                     device `pixma:04A91710_287A6D' is a CANON Canon PIXMA MG2520 multi-function peripheral\n";

        let scanners = parse_scanimage_list(input);

        assert_eq!(
            scanners,
            vec![
                SaneScannerDevice {
                    id: "escl:Brother DCP-L2540DW series [B1234]".to_string(),
                    description: "Brother scanner".to_string(),
                },
                SaneScannerDevice {
                    id: "pixma:04A91710_287A6D".to_string(),
                    description: "CANON Canon PIXMA MG2520 multi-function peripheral".to_string(),
                },
            ]
        );
    }

    #[test]
    fn ignores_non_device_lines() {
        let input = "No scanners were identified.\nTry scanimage -L";
        let scanners = parse_scanimage_list(input);
        assert!(scanners.is_empty());
    }

    #[test]
    fn parses_empty_input() {
        let scanners = parse_scanimage_list("");
        assert!(scanners.is_empty());
    }

    #[test]
    fn parses_single_device() {
        let input = "device `usb:001:002' is a Test Scanner Inc. Model X100\n";
        let scanners = parse_scanimage_list(input);
        assert_eq!(scanners.len(), 1);
        assert_eq!(scanners[0].id, "usb:001:002");
    }

    #[test]
    fn parses_device_with_malformed_description() {
        let input = "device `usb:001:002'\n";
        let scanners = parse_scanimage_list(input);
        assert!(scanners.is_empty());
    }

    #[test]
    fn parses_device_with_empty_id() {
        let input = "device `` is a Test Scanner\n";
        let scanners = parse_scanimage_list(input);
        assert!(scanners.is_empty());
    }

    #[test]
    fn parses_multiple_devices_mixed() {
        let input = "Some random line\n\
                     device `net:192.168.1.100' is a Network Scanner Pro\n\
                     More random text\n\
                     device `usb:002:005' is a USB Scanner Basic\n";
        let scanners = parse_scanimage_list(input);
        assert_eq!(scanners.len(), 2);
    }

    #[test]
    fn sane_scanner_device_clone() {
        let device = SaneScannerDevice {
            id: "usb:001:002".to_string(),
            description: "Test Scanner".to_string(),
        };
        let cloned = device.clone();
        assert_eq!(device.id, cloned.id);
        assert_eq!(device.description, cloned.description);
    }

    #[test]
    fn sane_scanner_device_partial_eq() {
        let a = SaneScannerDevice {
            id: "usb:001:002".to_string(),
            description: "Test Scanner".to_string(),
        };
        let b = SaneScannerDevice {
            id: "usb:001:002".to_string(),
            description: "Test Scanner".to_string(),
        };
        let c = SaneScannerDevice {
            id: "usb:001:003".to_string(),
            description: "Test Scanner".to_string(),
        };
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn sane_scan_output_debug() {
        let output = SaneScanOutput {
            bytes_written: 1024,
        };
        let debug_str = format!("{:?}", output);
        assert!(debug_str.contains("SaneScanOutput"));
    }
}
