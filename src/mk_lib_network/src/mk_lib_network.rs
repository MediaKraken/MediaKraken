use reqwest::header::CONTENT_TYPE;
use reqwest::header::USER_AGENT;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest::{Error, Response};
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::Cursor;
use std::io::Write;
use std::path::PathBuf;
use std::str;
use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};
use tokio::time::Duration;
use bytes::Bytes;
use futures_util::StreamExt;

pub async fn is_url_available(url: &str) -> bool {
    let client = Client::new();
    // Try HEAD first (no body download)
    if let Ok(resp) = client.head(url).send().await {
        return resp.status().is_success();
    }
    // Fallback to GET (still async, body not read)
    client
        .get(url)
        .send()
        .await
        .map(|resp| resp.status().is_success())
        .unwrap_or(false)
}

pub async fn custom_headers(map: &HashMap<String, String>) -> HeaderMap {
    let mut headers = HeaderMap::new();
    for (key, value) in map.iter() {
        headers.insert(
            HeaderName::from_bytes(key.as_bytes()).unwrap(),
            HeaderValue::from_bytes(value.as_bytes()).unwrap(),
        );
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
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(100);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    let res: serde_json::Value = client
        .get(url)
        .timeout(Duration::from_secs(30))
        .header(CONTENT_TYPE, "Content-Type: application/json")
        .header(
            USER_AGENT,
            "User-Agent: Mozilla/5.0 (Windows NT 10.0; rv:91.0) Gecko/20100101 Firefox/91.0",
        )
        .send()
        .await?
        .json()
        .await?;
    Ok(res)
}

pub async fn mk_data_from_url(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let content = response.bytes().await?;
    Ok(str::from_utf8(&content).unwrap().to_string())
}

pub async fn mk_network_download_file_to_bytes(url: String) -> Result<Bytes, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let body_bytes = response.bytes().await?;
    Ok(body_bytes)
}

pub async fn mk_network_download_file_to_vec(url: String) -> Result<Vec<u8>, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?.to_vec();
    Ok(bytes)
}

pub async fn mk_download_file_from_url(
    url: String,
    file_name: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("url: {}", url);
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

pub async fn mk_download_file_from_url_stream(
    url: String,
    file_name: &str, // Changed to &str for better ergonomics
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Downloading from: {}", url);

    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    
    // Get the body as a stream of chunks
    let mut byte_stream = response.bytes_stream();

    while let Some(chunk) = byte_stream.next().await {
        let data = chunk?;
        file.write_all(&data)?;
    }

    println!("Download complete: {}", file_name);
    Ok(())
}

pub async fn mk_download_file_from_url_tokio(
    url: String,
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .user_agent("MediaKraken/0.0.1")
        .build()?;

    // 1. Handle Request Errors
    let mut res = client.get(&url).send().await.map_err(|e| {
        // Log here if needed: format!("Network error: {}", e)
        e
    })?;

    // 2. Handle File Creation Errors
    let file = tokio::fs::File::create(file_name).await.map_err(|e| {
        // Log here: format!("File system error: {}", e)
        e
    })?;
    
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
pub async fn mk_network_service_available(host_dns: &str, host_port: &str, wait_seconds: &str) {
    let mut command_string = "/mediakraken/wait-for-it-bash.sh";
    if std::path::Path::new("/mediakraken/wait-for-it-ash-busybox130.sh").exists() {
        command_string = "/mediakraken/wait-for-it-ash-busybox130.sh";
    } else if std::path::Path::new("/mediakraken/wait-for-it-ash.sh").exists() {
        command_string = "/mediakraken/wait-for-it-ash.sh";
    }
    std::process::Command::new(command_string)
        .arg("-h")
        .arg(host_dns)
        .arg("-p")
        .arg(host_port)
        .arg("-t")
        .arg(wait_seconds)
        .spawn()
        .unwrap();
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
