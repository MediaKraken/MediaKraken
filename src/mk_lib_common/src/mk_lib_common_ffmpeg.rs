use std::process::{Command, Stdio};

pub async fn mk_common_ffmpeg_get_info(
    media_file: &str,
) -> Result<serde_json::Value, std::io::Error> {
    let output = Command::new("ffprobe")
        .args([
            "-hide_banner",
            "-show_format",
            "-show_streams",
            "-show_chapters",
            "-print_format",
            "json",
            &media_file,
        ])
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let json_output = serde_json::from_str(&stdout).unwrap();
    Ok(json_output)
}
