use bytes::Bytes;
use futures_util::StreamExt;
use reqwest::Client;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::USER_AGENT;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest_middleware::ClientBuilder;
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use std::collections::HashMap;
use std::str;
use std::sync::LazyLock;
use tokio::io::AsyncWriteExt;
use tokio::time::Duration;

fn sanitize_url_for_logging(url: &str) -> String {
    // Remove query parameters that may contain tokens/credentials
    let truncated = if let Some(pos) = url.find('?') {
        &url[..pos]
    } else {
        url
    };
    // Truncate very long URLs to prevent log flooding
    if truncated.len() > 200 {
        format!("{}... (truncated)", &truncated[..200])
    } else {
        truncated.to_string()
    }
}

static SHARED_HTTP_CLIENT: LazyLock<Client> = LazyLock::new(Client::new);

pub async fn custom_headers(map: &HashMap<String, String>) -> HeaderMap {
    #[allow(dead_code)]
    pub async fn is_url_available(url: &str) -> bool {
        // Try HEAD first (no body download)
        match SHARED_HTTP_CLIENT.head(url).send().await {
            Ok(resp) => {
                resp.status().is_success()
            }
            Err(_) => {
                // If HEAD fails, fallback to GET
                match SHARED_HTTP_CLIENT.get(url).send().await {
                    Ok(resp) => {
                        resp.status().is_success()
                    }
                    Err(_) => {
                        false
                    }
                }
            }
        }
    }
    let mut headers = HeaderMap::new();
    for (key, value) in map.iter() {
        if let (Ok(header_name), Ok(header_value)) = (
            HeaderName::from_bytes(key.as_bytes()),
            HeaderValue::from_bytes(value.as_bytes()),
        ) {
            headers.insert(header_name, header_value);
        }
    }
    headers
}

pub async fn mk_data_from_url_to_json_custom_headers(
    url: String,
    custom_headers: HeaderMap,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().build()?;
    let res: serde_json::Value = client
        .get(url)
        .timeout(Duration::from_secs(30))
        .headers(custom_headers)
        .send()
        .await?
        .json()
        .await?;
    Ok(res)
}

pub async fn mk_data_from_url_to_json(
    url: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // 100 retries with exponential backoff would stall callers for hours on a
    // permanent upstream failure; 3 attempts is plenty for transient blips.
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    let res: serde_json::Value = client
        .get(url)
        .timeout(Duration::from_secs(30))
        .header(CONTENT_TYPE, "application/json")
        .header(
            USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; rv:91.0) Gecko/20100101 Firefox/91.0",
        )
        .send()
        .await?
        .json()
        .await?;
    Ok(res)
}

pub async fn mk_data_from_url(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let response = SHARED_HTTP_CLIENT.get(url).send().await?;
    let content = response.bytes().await?;
    Ok(str::from_utf8(&content)?.to_string())
}

pub async fn mk_network_download_file_to_bytes(
    url: String,
) -> Result<Bytes, Box<dyn std::error::Error>> {
    let response = SHARED_HTTP_CLIENT.get(url).send().await?;
    let body_bytes = response.bytes().await?;
    Ok(body_bytes)
}

pub async fn mk_network_download_file_to_vec(
    url: String,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let response = SHARED_HTTP_CLIENT.get(url).send().await?;
    let bytes = response.bytes().await?.to_vec();
    Ok(bytes)
}

pub async fn mk_download_file_from_url(
    url: String,
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let safe_url = sanitize_url_for_logging(&url);
    eprintln!("[DEBUG] Downloading file from: {}", safe_url);
    let response = SHARED_HTTP_CLIENT.get(url).send().await?;
    let mut file = tokio::fs::File::create(file_name).await?;
    file.write_all(&response.bytes().await?).await?;
    file.flush().await?;
    Ok(())
}

pub async fn mk_download_file_from_url_stream(
    url: String,
    file_name: &str, // Changed to &str for better ergonomics
) -> Result<(), Box<dyn std::error::Error>> {
    let safe_url = sanitize_url_for_logging(&url);
    eprintln!("[DEBUG] Downloading file from: {}", safe_url);

    let response = SHARED_HTTP_CLIENT.get(url).send().await?;
    let mut file = tokio::fs::File::create(file_name).await?;

    // Get the body as a stream of chunks
    let mut byte_stream = response.bytes_stream();

    while let Some(chunk) = byte_stream.next().await {
        let data = chunk?;
        file.write_all(&data).await?;
    }

    file.flush().await?;

    eprintln!("[DEBUG] Download complete: {}", file_name);
    Ok(())
}

pub async fn mk_download_file_from_url_tokio(
    url: String,
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().user_agent("MediaKraken/0.0.1").build()?;

    // 1. Handle Request Errors
    let mut res = client.get(&url).send().await?;

    // 2. Handle File Creation Errors
    let file = tokio::fs::File::create(file_name).await?;

    let mut writer = tokio::io::BufWriter::new(file);

    // 3. Stream and Write
    while let Some(chunk) = res.chunk().await? {
        writer.write_all(&chunk).await?;
    }

    // Ensure all buffers are pushed to disk
    writer.flush().await?;

    Ok(())
}

// wait_seconds - 120 typically
pub async fn mk_network_service_available(
    host_dns: &str,
    host_port: &str,
    wait_seconds: &str,
) -> Result<(), std::io::Error> {
    let mut command_string = "/mediakraken/wait-for-it-bash.sh";
    if std::path::Path::new("/mediakraken/wait-for-it-ash-busybox130.sh").exists() {
        command_string = "/mediakraken/wait-for-it-ash-busybox130.sh";
    } else if std::path::Path::new("/mediakraken/wait-for-it-ash.sh").exists() {
        command_string = "/mediakraken/wait-for-it-ash.sh";
    }
    tokio::process::Command::new(command_string)
        .arg("-h")
        .arg(host_dns)
        .arg("-p")
        .arg(host_port)
        .arg("-t")
        .arg(wait_seconds)
        .spawn()?;
    Ok(())
}

// cargo test -- --show-output
#[cfg(test)]
mod test_mk_lib_network {
    use super::*;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_mk_data_from_url() {
        let res = aw!(mk_data_from_url(
            "https://github.com/MediaKraken/MediaKraken_Deployment/raw/master/LICENSE".to_string()
        ));
        assert!(res.is_ok());
    }

    #[test]
    fn test_mk_download_file_from_url() {
        let res = aw!(mk_download_file_from_url(
            "https://github.com/MediaKraken/MediaKraken_Deployment/raw/master/LICENSE".to_string(),
            &"license.md".to_string()
        ));
        assert!(res.is_ok());
    }
}
