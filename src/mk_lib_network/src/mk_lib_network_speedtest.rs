// apt install speedtest-cli

use std::process::Command;

fn mk_lib_network_speedtest() -> (Option<String>, Option<String>) {
    let output = Command::new("speedtest-cli")
        .output()
        .expect("Failed to execute command");
    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut speed_download = None;
    let mut speed_upload = None;
    for speed_list in output_str.split('\n') {
        if speed_list.contains("Download: ") {
            speed_download = Some(speed_list.splitn(2, ' ').nth(1).unwrap().to_string());
        }
        if speed_list.contains("Upload: ") {
            speed_upload = Some(speed_list.splitn(2, ' ').nth(1).unwrap().to_string());
        }
    }
    (speed_download, speed_upload)
}
