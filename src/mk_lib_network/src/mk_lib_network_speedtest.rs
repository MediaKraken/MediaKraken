// apt install speedtest-cli

use std::process::Command;

#[allow(dead_code)]
fn mk_lib_network_speedtest() -> Result<(Option<String>, Option<String>), Box<dyn std::error::Error>> {
    let output = Command::new("speedtest-cli")
        .output()
        .map_err(|e| format!("Failed to execute command: {e}"))?;
    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut speed_download = None;
    let mut speed_upload = None;
    for speed_list in output_str.split('\n') {
        if speed_list.contains("Download: ") {
            speed_download = Some(speed_list.split_once(' ').ok_or("no space in download line")?.1.to_string());
        }
        if speed_list.contains("Upload: ") {
            speed_upload = Some(speed_list.split_once(' ').ok_or("no space in upload line")?.1.to_string());
        }
    }
    Ok((speed_download, speed_upload))
}
