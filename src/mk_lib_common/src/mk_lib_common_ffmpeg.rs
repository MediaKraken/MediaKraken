use std::io::{Error, ErrorKind};
use std::process::Stdio;
use tokio::process::Command;

pub async fn mk_common_ffmpeg_get_info(
    media_file: &str,
) -> Result<serde_json::Value, std::io::Error> {
    if media_file.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "media_file must not be empty",
        ));
    }
    if media_file.starts_with('-') {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "media_file must not start with '-' (potential option injection)",
        ));
    }

    let output = Command::new("ffprobe")
        .args([
            "-hide_banner",
            "-show_format",
            "-show_streams",
            "-show_chapters",
            "-print_format",
            "json",
            "-i",
            media_file,
        ])
        .stdout(Stdio::piped())
        .output()
        .await?;

    if !output.status.success() {
        return Err(Error::other(format!(
            "ffprobe failed for '{media_file}' with status {}",
            output.status
        )));
    }

    serde_json::from_slice(&output.stdout).map_err(|err| Error::new(ErrorKind::InvalidData, err))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffmpeg_empty_media_file_rejected() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(mk_common_ffmpeg_get_info(""));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidInput);
    }

    #[test]
    fn test_ffmpeg_dangerous_media_file_rejected() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(mk_common_ffmpeg_get_info("-i malicious"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidInput);
    }

    #[test]
    fn test_ffmpeg_dash_starting_file_rejected() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(mk_common_ffmpeg_get_info("--help"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidInput);
    }
}
