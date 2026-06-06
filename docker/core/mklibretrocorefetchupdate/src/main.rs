use mk_lib_hash;
use mk_lib_network;
use mk_lib_rabbitmq;
use std::collections::HashMap;
use std::error::Error;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use tokio::sync::Notify;
use walkdir::WalkDir;

async fn process_cores(rabbit_channel: &amqprs::channel::Channel) -> Result<(), Box<dyn Error>> {
    let mut emulation_cores = HashMap::new();
    let walker = WalkDir::new("/mediakraken/emulation/cores").into_iter();
    for entry in walker
        .filter_entry(|e| !is_hidden(e.file_name()))
        .filter_map(Result::ok)
        .filter(|d| d.path().extension() == Some(OsStr::from_bytes(b"zip")))
        .filter(|e| !e.file_type().is_dir())
    {
        let file_name = entry.path().display().to_string();
        let crc = mk_lib_hash::mk_lib_hash_crc32::mk_file_hash_crc32(&file_name)
            .await?;
        emulation_cores.insert(file_name, crc);
    }

    let stable_root = "http://buildbot.libretro.com/stable/";
    let stable_index = mk_lib_network::mk_lib_network::mk_data_from_url(stable_root.to_string()).await?;
    let latest_version = latest_stable_version(&stable_index).ok_or("No stable version found")?;
    let libtro_url = format!("{}{}/linux/x86_64/", stable_root, latest_version);
    let fetch_result = mk_lib_network::mk_lib_network::mk_data_from_url(format!(
        "{}{}",
        &libtro_url, ".index-extended"
    )).await?;

    for libretro_core in fetch_result.split('\n') {
        if libretro_core.len() > 0 {
            let mut parts = libretro_core.splitn(3, " ");
            let core_date = parts.next().ok_or("Missing core_date")?;
            let core_crc32 = parts.next().ok_or("Missing core_crc32")?;
            let core_name = parts.next().ok_or("Missing core_name")?;
            let path_core_name = format!(
                "/mediakraken/emulation/cores/{}",
                core_name.replace(".zip", "")
            );
            if emulation_cores.contains_key(&path_core_name) {
                if emulation_cores[&path_core_name] != core_crc32 {
                    mk_lib_network::mk_lib_network::mk_download_file_from_url(
                        format!("{}{}", &libtro_url, core_name),
                        &format!("/mediakraken/emulation/cores/{}", core_name),
                    ).await?;
                }
            } else {
                mk_lib_network::mk_lib_network::mk_download_file_from_url(
                    format!("{}{}", &libtro_url, core_name),
                    &format!("/mediakraken/emulation/cores/{}", core_name),
                ).await?;
            }
        }
    }
    Ok(())
}

fn is_hidden(file_name: &std::ffi::OsStr) -> bool {
    file_name
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn parse_version_components(version: &str) -> Option<Vec<u32>> {
    let parsed = version
        .split('.')
        .map(str::parse::<u32>)
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    if parsed.is_empty() {
        return None;
    }
    Some(parsed)
}

fn latest_stable_version(index_html: &str) -> Option<String> {
    index_html
        .split("href=\"")
        .skip(1)
        .filter_map(|segment| segment.split('\"').next())
        .filter_map(|href| href.strip_suffix('/'))
        .filter(|version| version.chars().all(|c| c.is_ascii_digit() || c == '.'))
        .filter_map(|version| parse_version_components(version).map(|parts| (parts, version)))
        .max_by(|(left, _), (right, _)| left.cmp(right))
        .map(|(_, version)| version.to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mklibretrocorefetchupdate").await?;

    let mut rabbit_consumer = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer(
        "mklibretrocorefetchupdate",
        &rabbit_channel,
    )
    .await?;

   tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(payload) = msg.content {
                let _parsed: serde_json::Value = match serde_json::from_slice(&payload) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("Failed to parse JSON: {}", e);
                        continue;
                    }
                };
                if let Err(e) = process_cores(&rabbit_channel).await {
                    eprintln!("Core processing failed: {}", e);
                }
                let _result = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                    &rabbit_channel,
                    msg.deliver.map(|d| d.delivery_tag()).unwrap_or(0),
                )
                .await;
            }
        }
    });
    let guard = Notify::new();
    guard.notified().await;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_hidden_dotfile() {
        assert!(is_hidden(OsStr::new(".hidden")));
    }

    #[test]
    fn test_is_hidden_normal_file() {
        assert!(!is_hidden(OsStr::new("regular.txt")));
    }

    #[test]
    fn test_is_hidden_dotdot() {
        assert!(is_hidden(OsStr::new("..secret")));
    }

    #[test]
    fn test_parse_version_components_valid() {
        let parts = parse_version_components("1.2.3");
        assert_eq!(parts, Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_parse_version_components_single() {
        let parts = parse_version_components("1");
        assert_eq!(parts, Some(vec![1]));
    }

    #[test]
    fn test_parse_version_components_two_parts() {
        let parts = parse_version_components("1.2");
        assert_eq!(parts, Some(vec![1, 2]));
    }

    #[test]
    fn test_parse_version_components_invalid() {
        let parts = parse_version_components("1.2.x");
        assert!(parts.is_none());
    }

    #[test]
    fn test_parse_version_components_empty() {
        let parts = parse_version_components("");
        assert!(parts.is_none());
    }

    #[test]
    fn test_parse_version_components_negative() {
        let parts = parse_version_components("1.-2.3");
        assert!(parts.is_none());
    }

    #[test]
    fn test_latest_stable_version_simple() {
        let html = r#"<html><body>
            <a href="1.0.0/">v1.0.0</a>
            <a href="1.9.0/">v1.9.0</a>
            <a href="1.10.0/">v1.10.0</a>
        </body></html>"#;
        assert_eq!(latest_stable_version(html), Some("1.10.0".to_string()));
    }

    #[test]
    fn test_latest_stable_version_no_trailing_slash() {
        let html = r#"<html><body>
            <a href="1.0.0">v1.0.0</a>
        </body></html>"#;
        assert!(latest_stable_version(html).is_none());
    }

    #[test]
    fn test_latest_stable_version_mixed_content() {
        let html = r#"<html><body>
            <a href="stable/1.0.0/">old</a>
            <a href="1.9.0/">latest</a>
            <a href="not-a-version/">invalid</a>
        </body></html>"#;
        assert_eq!(latest_stable_version(html), Some("1.9.0".to_string()));
    }

    #[test]
    fn test_latest_stable_version_empty() {
        assert!(latest_stable_version("").is_none());
    }

    #[test]
    fn test_latest_stable_version_no_links() {
        let html = "<html><body>No links here</body></html>";
        assert!(latest_stable_version(html).is_none());
    }

    #[test]
    fn test_latest_stable_version_alpha_chars_rejected() {
        let html = r#"<html><body>
            <a href="1.0.0a/">invalid</a>
            <a href="1.0.0/">valid</a>
        </body></html>"#;
        assert_eq!(latest_stable_version(html), Some("1.0.0".to_string()));
    }

    #[test]
    fn test_latest_stable_version_hyphen_rejected() {
        let html = r#"<html><body>
            <a href="1.0.0-beta/">invalid</a>
            <a href="1.0.0/">valid</a>
        </body></html>"#;
        assert_eq!(latest_stable_version(html), Some("1.0.0".to_string()));
    }

    #[test]
    fn test_latest_stable_version_large_versions() {
        let html = r#"<html><body>
            <a href="0.1.0/">old</a>
            <a href="20.30.40/">new</a>
        </body></html>"#;
        assert_eq!(latest_stable_version(html), Some("20.30.40".to_string()));
    }
}
