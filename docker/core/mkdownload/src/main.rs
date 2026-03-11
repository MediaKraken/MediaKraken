use mk_lib_database;
use mk_lib_network;
use mk_lib_rabbitmq;
use serde_json::{Value, json};
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use tokio::sync::{Notify, Semaphore};

const DEFAULT_MAX_CONCURRENT_DOWNLOADS: usize = 4;

async fn process_message(json_message: Value, option_config_json: &Value) {
    if json_message["Type"].to_string() == "File" {
        // do NOT remove the header.....this is the SAVE location
        mk_lib_network::mk_lib_network::mk_download_file_from_url(
            json_message["URL"].to_string(),
            &json_message["Local Save Path"].to_string(),
        )
        .await
        .unwrap();
    } else if json_message["Type"].to_string() == "Youtube" {
        if validator::ValidateUrl::validate_url(&json_message["URL"].to_string()) {
            //println!("downloaded video to {:?}", rustube::download_best_quality(&json_message["URL"].to_string()).await.unwrap());
            return;
        }
        // TODO log error by user requested
    } else if json_message["Type"].to_string() == "Subtitle" {
        let _output = Command::new("subliminal")
            .args(["-l", "en", &json_message["Data"].as_str().unwrap()])
            .stdout(Stdio::piped())
            .output()
            .unwrap();
    } else if json_message["Type"].to_string() == "Twitch" {
        if validator::ValidateUrl::validate_url(&json_message["URL"].to_string()) {
            let _res = mk_lib_network::mk_lib_network::mk_download_file_from_url_tokio(
                json_message["URL"].to_string(),
                &json_message["Local Save Path"].to_string(),
            )
            .await;
        }
        // TODO log error by user requested
        // } else if json_message["Type"].to_string() == "DigitalUPCNet" {
        //     let sheet_data = mk_lib_metadata::mk_lib_metadata_provider_google_sheets::provider_google_sheets_fetch(
        //         "1po70GCN9JUwrWgycMueNfxpEvBjLd7DQkiMRUQFFsL8".to_string(),
        //         "tsv".to_string(),
        //     )
        //     .await
        //     .unwrap();
        //     // process sheet data TODO, this might be worthless d2d only
        //     let mut rdr = csv::Reader::from_reader(sheet_dataq.as_bytes());
        //     for result in rdr.deserialize() {
        //         let record: DigitalUPCNetRecord = result?;
        //         println!("{:?}", record);
        //         // TODO "one-time" load.....do this BEFORE upc master list
        //     }
        // } else if json_message["Type"].to_string() == "UPCMasterList" {
        //     let sheet_data = mk_lib_metadata::mk_lib_metadata_provider_google_sheets::provider_google_sheets_fetch(
        //         "1IgK7tIEKngP59PUOs_lsF4P1hbSRIG71tgxncpu1Mws".to_string(),
        //         "tsv".to_string(),
        //     )
        //     .await
        //     .unwrap();
        //     // process sheet data, this might be worthless d2d only
        //     let mut rdr = csv::Reader::from_reader(sheet_dataq.as_bytes());
        //     for result in rdr.deserialize() {
        //         let record: UPCMasterNetRecord = result?;
        //         println!("{:?}", record);
        //         // TODO "one-time" load
        //     }
    } else if json_message["Type"].to_string() == "Dosage" {
        // This saves to ./Comics
        let output = Command::new("dosage")
            .args(["--adult", &json_message["Data"].as_str().unwrap()])
            .stdout(Stdio::piped())
            .output()
            .unwrap();
        let _stdout = String::from_utf8(output.stdout).unwrap();
        if json_message["Data"].as_str().unwrap() == "--list" {
            // TODO parse list and store the strips, see notes in data example
        }
    } else if json_message["Type"].to_string() == "HDTrailers" {
        // try to grab the RSS feed itself
        let data: serde_json::Value = serde_json::from_str(
            &mk_lib_network::mk_lib_network::mk_data_from_url(
                "http://feeds.hd-trailers.net/hd-trailers".to_string(),
            )
            .await
            .unwrap(),
        )
        .unwrap();
        let an_array = data["rss"]["channel"]["item"].as_array().unwrap();
        for item in an_array.iter() {
            if (item["title"].to_string().contains("(Trailer")
                && option_config_json["Metadata"]["Trailer"]["Trailer"] == true)
                || (item["title"].to_string().contains("(Behind")
                    && option_config_json["Metadata"]["Trailer"]["Behind"] == true)
                || (item["title"].to_string().contains("(Clip")
                    && option_config_json["Metadata"]["Trailer"]["Clip"] == true)
                || (item["title"].to_string().contains("(Featurette")
                    && option_config_json["Metadata"]["Trailer"]["Featurette"] == true)
                || (item["title"].to_string().contains("(Carpool")
                    && option_config_json["Metadata"]["Trailer"]["Carpool"] == true)
            {
                let download_link = item["enclosure"]["@url"].to_string();
                // do NOT remove the header.....this is the SAVE location
                // TODO use image directory format
                let file_save_name = format!(
                    "/mediakraken/metadata/meta/trailer/{:?}",
                    download_link.rsplitn(1, "/")
                );
                // verify it doesn't exist in meta folder before downloading
                if !Path::new(&file_save_name).exists() {
                    mk_lib_network::mk_lib_network::mk_download_file_from_url(
                        download_link.to_string(),
                        &file_save_name.to_string(),
                    )
                    .await
                    .unwrap();
                }
            }
        }
    }
}

// #[derive(Debug, serde::Deserialize)]
// struct DigitalUPCNetRecord {
//     title: String,
//     year: String,
//     quality: String,
//     upc: String,
//     notes: Option<String>,
//     fah_id: Option<String>,
// }

// #[derive(Debug, serde::Deserialize)]
// struct UPCMasterNetRecord {
//     upc: String,
//     title: String,
//     description: Option<String>,
//     link: String,
//     notes: Option<String>,
//     upc_type: String,
//     year: String,
//     genres: String,
//     rated: String,
//     length: String,
//     added: String,
//     alt_upc: Option<String>,
//     nw_bluray_upc: Option<String>,
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check
    let (sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(50, 120)
            .await
            .unwrap();
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
        .await
        .unwrap();
    let option_config_json: Value =
        mk_lib_database::mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool_ro)
            .await
            .unwrap();

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkdownload")
            .await
            .unwrap();

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mkdownload", &rabbit_channel)
            .await
            .unwrap();

    let max_concurrent_downloads = env::var("MKDOWNLOAD_CONCURRENT_WORKERS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_MAX_CONCURRENT_DOWNLOADS);

    let worker_limit = Arc::new(Semaphore::new(max_concurrent_downloads));
    let option_config_json = Arc::new(option_config_json);
    let rabbit_channel = Arc::new(rabbit_channel);

    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(payload) = msg.content {
                let json_message: Value =
                    serde_json::from_str(&String::from_utf8_lossy(&payload)).unwrap();
                let delivery_tag = msg.deliver.unwrap().delivery_tag();
                let worker_limit = Arc::clone(&worker_limit);
                let option_config_json = Arc::clone(&option_config_json);
                let rabbit_channel = Arc::clone(&rabbit_channel);

                tokio::spawn(async move {
                    if let Ok(_permit) = worker_limit.acquire_owned().await {
                        process_message(json_message, option_config_json.as_ref()).await;
                        let _result = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                            &rabbit_channel,
                            delivery_tag,
                        )
                        .await;
                    }
                });
            }
        }
    });

    let guard = Notify::new();
    guard.notified().await;
    Ok(())
}

async fn process_ia_trailer_request(json_message: &Value) -> Result<(), Box<dyn Error>> {
    let output_dir = json_message
        .get("OutputDir")
        .and_then(Value::as_str)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(IA_DEFAULT_OUTPUT_DIR));
    let state_file = json_message
        .get("StateFile")
        .and_then(Value::as_str)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(IA_DEFAULT_STATE_FILE));
    let max_pages = json_message
        .get("MaxPages")
        .and_then(Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
        .unwrap_or(IA_DEFAULT_MAX_PAGES);

    tokio::fs::create_dir_all(&output_dir).await?;

    let mut state = ia_load_state(&state_file).await?;
    let client = Client::builder()
        .user_agent("mediakraken-mkdownload-ia-trailer/0.1 (+https://archive.org)")
        .build()?;

    for page in 1..=max_pages {
        let docs = ia_search_trailer_items(&client, page).await?;
        if docs.is_empty() {
            break;
        }

        for doc in docs {
            if state.downloaded_ids.contains(&doc.identifier) {
                continue;
            }

            if let Some(filename) = ia_fetch_best_trailer_file(&client, &doc.identifier).await? {
                let saved_path =
                    ia_download_file(&client, &doc.identifier, &filename, &output_dir).await?;
                state.downloaded_ids.insert(doc.identifier.clone());
                state.downloaded_files.insert(
                    doc.identifier.clone(),
                    saved_path.to_string_lossy().to_string(),
                );
                ia_save_state(&state_file, &state).await?;
                sleep(IA_DOWNLOAD_DELAY).await;
            }
        }

        sleep(IA_SEARCH_DELAY).await;
    }

    Ok(())
}

async fn ia_search_trailer_items(
    client: &Client,
    page: usize,
) -> Result<Vec<IASearchDoc>, Box<dyn Error>> {
    let query = "mediatype:movies AND subject:trailer";
    let url = format!(
        "https://archive.org/advancedsearch.php?q={}&fl[]=identifier&sort[]=downloads+desc&rows={}&page={}&output=json",
        urlencoding::encode(query),
        IA_SEARCH_ROWS,
        page
    );
    let response: IASearchResponse = ia_fetch_json_with_backoff(client, &url).await?;
    Ok(response.response.docs)
}

async fn ia_fetch_best_trailer_file(
    client: &Client,
    identifier: &str,
) -> Result<Option<String>, Box<dyn Error>> {
    let metadata_url = format!(
        "https://archive.org/metadata/{}",
        urlencoding::encode(identifier)
    );
    let metadata: IAMetadataResponse = ia_fetch_json_with_backoff(client, &metadata_url).await?;

    let preferred_extensions = [".mp4", ".m4v", ".mkv", ".webm"];
    let mut best: Option<String> = None;

    for file in metadata.files {
        let Some(name) = file.name else {
            continue;
        };

        let name_lower = name.to_ascii_lowercase();
        let format_lower = file.format.unwrap_or_default().to_ascii_lowercase();
        let likely_trailer = name_lower.contains("trailer") || format_lower.contains("trailer");
        let video_format = preferred_extensions
            .iter()
            .any(|ext| name_lower.ends_with(ext))
            || format_lower.contains("mpeg4")
            || format_lower.contains("h.264")
            || format_lower.contains("matroska")
            || format_lower.contains("webm");

        if likely_trailer && video_format {
            return Ok(Some(name));
        }

        if best.is_none() && video_format {
            best = Some(name);
        }
    }

    Ok(best)
}

async fn ia_download_file(
    client: &Client,
    identifier: &str,
    filename: &str,
    output_dir: &Path,
) -> Result<PathBuf, Box<dyn Error>> {
    let url = format!(
        "https://archive.org/download/{}/{}",
        urlencoding::encode(identifier),
        urlencoding::encode(filename)
    );

    let mut response = ia_get_with_backoff(client, &url).await?;
    let output_name = ia_sanitize_filename(identifier, filename);
    let output_path = output_dir.join(output_name);
    let mut file = tokio::fs::File::create(&output_path).await?;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
    }
    file.flush().await?;

    Ok(output_path)
}

async fn ia_fetch_json_with_backoff<T>(client: &Client, url: &str) -> Result<T, Box<dyn Error>>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let response = ia_get_with_backoff(client, url).await?;
    Ok(response.json::<T>().await?)
}

async fn ia_get_with_backoff(
    client: &Client,
    url: &str,
) -> Result<reqwest::Response, Box<dyn Error>> {
    let mut attempt = 0;
    loop {
        attempt += 1;
        let response = client.get(url).send().await?;

        if response.status() == StatusCode::TOO_MANY_REQUESTS {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or(5);

            if attempt > IA_MAX_RETRIES {
                return Err(
                    format!("rate limited after {IA_MAX_RETRIES} retries for {url}").into(),
                );
            }

            sleep(Duration::from_secs(retry_after)).await;
            continue;
        }

        if !response.status().is_success() {
            return Err(format!("request failed ({}): {url}", response.status()).into());
        }

        return Ok(response);
    }
}

fn ia_sanitize_filename(identifier: &str, filename: &str) -> String {
    let source = format!("{identifier}_{filename}");
    source
        .chars()
        .map(|value| {
            if value.is_ascii_alphanumeric() || value == '.' || value == '-' || value == '_' {
                value
            } else {
                '_'
            }
        })
        .collect()
}

async fn ia_load_state(path: &Path) -> Result<IATrailerState, Box<dyn Error>> {
    match tokio::fs::read(path).await {
        Ok(bytes) => Ok(serde_json::from_slice(&bytes)?),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(IATrailerState::default()),
        Err(error) => Err(error.into()),
    }
}

async fn ia_save_state(path: &Path, state: &IATrailerState) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_vec_pretty(state)?;
    tokio::fs::write(path, data).await?;
    Ok(())
}
