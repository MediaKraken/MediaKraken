use mk_lib_logging::mk_lib_logging;
use reqwest::header::CONTENT_TYPE;
use reqwest::header::USER_AGENT;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::RetryTransientMiddleware;
use serde_json::json;
use std::collections::HashMap;
use std::io::Cursor;
use std::str;
use stdext::function_name;
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
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!(), "URL": url }),
        )
        .await
        .unwrap();
    }
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
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!(), "URL": url }),
        )
        .await
        .unwrap();
    }
    let response = reqwest::get(url).await?;
    let content = response.bytes().await?;
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "content": str::from_utf8(&content).unwrap().to_string() }),
        )
        .await
        .unwrap();
    }
    Ok(str::from_utf8(&content).unwrap().to_string())
}

pub async fn mk_download_file_from_url(
    url: String,
    file_name: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(std::module_path!(), json!({ "url": url }))
            .await
            .unwrap();
    }
    println!("url: {}", url);
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

// wait_seconds - 120 typically
pub async fn mk_network_service_available(host_dns: &str, host_port: &str, wait_seconds: &str) {
    #[cfg(debug_assertions)]
    {
        mk_lib_logging::mk_logging_post_elk(
            std::module_path!(),
            json!({ "Function": function_name!() }),
        )
        .await
        .unwrap();
    }
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
