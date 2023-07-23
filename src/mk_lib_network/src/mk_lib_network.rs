use reqwest::header::CONTENT_TYPE;
use reqwest::header::USER_AGENT;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::RetryTransientMiddleware;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::Cursor;
use std::io::Write;
use std::path::PathBuf;
use std::str;
use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};
use tokio::time::Duration;

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
    let retry_policy = reqwest_retry::policies::ExponentialBackoff {
        /// How many times the policy will tell the middleware to retry the request.
        max_n_retries: 100,
        min_retry_interval: std::time::Duration::from_secs(30),
        max_retry_interval: std::time::Duration::from_secs(300),
        backoff_exponent: 2,
    };
    let retry_transient_middleware = RetryTransientMiddleware::new_with_policy(retry_policy);
    let client = ClientBuilder::new(Client::new())
        .with(retry_transient_middleware)
        .build();
    //let client = reqwest::Client::builder().build()?;
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

pub async fn mk_download_file_from_url_tokio(
    url: String,
    file_name: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .user_agent("MediaKraken/0.0.1")
        .build()
        .expect("Could not build client");
    // get response
    let res = if let Ok(mut res) = client.get(&url).send().await {
        // Try to open file
        if let Ok(file) = tokio::fs::File::create(&file_name).await {
            let mut writer = tokio::io::BufWriter::new(file);
            // Write all bytes to file
            loop {
                match res.chunk().await {
                    Ok(Some(bytes)) => {
                        writer.write_all(&bytes).await;
                    }
                    Ok(None) => {
                        writer.flush().await;
                        break;
                    }
                    Err(e) => {
                        #[cfg(debug_assertions)]
                        {
                            // mk_lib_logging::mk_logging_post_elk(
                            //     std::module_path!(),
                            //     json!({ format!("Could not get bytes from data: {:?}", &e ): url }),
                            // )
                            // .await
                            // .unwrap();
                        }
                    }
                }
            }
        } else {
            #[cfg(debug_assertions)]
            {
                // mk_lib_logging::mk_logging_post_elk(
                //     std::module_path!(),
                //     json!({ format!("Could not open file for writing: {:?}", &file_name ): url }),
                // )
                // .await
                // .unwrap();
            }
        }
    } else {
        if let Err(e) = client.get(&url).send().await {
            #[cfg(debug_assertions)]
            {
                // mk_lib_logging::mk_logging_post_elk(
                //     std::module_path!(),
                //     json!({ format!("File error for {:?} with error {:#?}", &file_name, &e ): url }),
                // )
                // .await
                // .unwrap();
            }
        }
    };
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
