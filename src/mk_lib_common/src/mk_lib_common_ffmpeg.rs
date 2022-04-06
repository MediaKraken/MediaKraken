#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::process::{Command, Stdio};

pub fn mk_common_ffmpeg_get_info(media_file: &str) -> Result<(String), std::io::Error> {
    let output = Command::new("ffprobe")
        .args(["-hide_banner", "-show_format", "-show_streams", "-show_chapters",
            "-print_format", "json", &media_file])
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    Ok(stdout)
}
