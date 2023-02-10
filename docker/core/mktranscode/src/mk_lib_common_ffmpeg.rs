#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

#[path = "mk_lib_logging.rs"]
mod mk_lib_logging;

use serde_json::json;
use std::process::{Command, Stdio};
use stdext::function_name;

pub async fn mk_common_ffmpeg_get_info(
    media_file: &str,
) -> Result<(serde_json::Value), std::io::Error> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
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
