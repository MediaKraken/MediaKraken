#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use reqwest::header::CONTENT_TYPE;
use reqwest::header::USER_AGENT;
use std::collections::HashMap;
use std::io::Cursor;
use std::io::Read;
use std::net::UdpSocket;
use std::str;

pub async fn mk_data_from_url_to_json(
    url: String,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    // Build the client using the builder pattern
    let client = reqwest::Client::builder().build()?;
    // Perform the actual execution of the network request
    let res: serde_json::Value = client
        .get(url)
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
    #[cfg(debug_assertions)]
    {
        println!("response: {:?}", response);
    }
    let content = response.bytes().await?;
    Ok(str::from_utf8(&content).unwrap().to_string())
}

pub async fn mk_download_file_from_url(
    url: String,
    file_name: &String,
) -> Result<bool, Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    {
        println!("waffles {}", url);
    }
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(true)
}

// wait_seconds - 120 typically
pub async fn mk_network_service_available(host_dns: &str, host_port: &str, wait_seconds: &str) {
    if std::path::Path::new("/mediakraken/wait-for-it-ash-busybox130.sh").exists() {
        std::process::Command::new("/mediakraken/wait-for-it-ash-busybox130.sh")
            .arg("-h")
            .arg(host_dns)
            .arg("-p")
            .arg(host_port)
            .arg("-t")
            .arg(wait_seconds)
            .spawn()
            .unwrap();
    } else if std::path::Path::new("/mediakraken/wait-for-it-ash.sh").exists() {
        std::process::Command::new("/mediakraken/wait-for-it-ash.sh")
            .arg("-h")
            .arg(host_dns)
            .arg("-p")
            .arg(host_port)
            .arg("-t")
            .arg(wait_seconds)
            .spawn()
            .unwrap();
    } else {
        std::process::Command::new("/mediakraken/wait-for-it-bash.sh")
            .arg("-h")
            .arg(host_dns)
            .arg("-p")
            .arg(host_port)
            .arg("-t")
            .arg(wait_seconds)
            .spawn()
            .unwrap();
    }
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
