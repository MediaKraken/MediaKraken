use mk_lib_database;
use mk_lib_network;
use mk_lib_rabbitmq;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::sync::Notify;
use tokio::time::sleep;

const IA_ADVANCED_SEARCH_URL: &str = "https://archive.org/advancedsearch.php";
const IA_METADATA_URL_PREFIX: &str = "https://archive.org/metadata/";
const IA_DEFAULT_QUERY: &str = "mediatype:(movies) AND collection:(feature_films)";
const IA_DEFAULT_REQUEST_DELAY_MS: u64 = 1000;
const IA_DEFAULT_DOWNLOAD_DELAY_MS: u64 = 2000;
const IA_DEFAULT_MAX_ITEMS: usize = 30;
const IA_DEFAULT_MAX_DOWNLOADS: usize = 5;

#[derive(Debug, Serialize, Deserialize, Default)]
struct IAMovieState {
    downloaded: HashSet<String>,
}

#[derive(Debug, Deserialize)]
struct IASearchResponse {
    response: IASearchDocs,
}

#[derive(Debug, Deserialize)]
struct IASearchDocs {
    docs: Vec<IASearchDoc>,
}

#[derive(Debug, Deserialize)]
struct IASearchDoc {
    identifier: String,
}

#[derive(Debug, Deserialize)]
struct IAMetadataResponse {
    files: Option<Vec<IAMetadataFile>>,
}

#[derive(Debug, Deserialize)]
struct IAMetadataFile {
    name: String,
    format: Option<String>,
}

#[derive(Debug)]
struct IAMovieConfig {
    query: String,
    output_dir: String,
    state_file: String,
    max_items: usize,
    max_downloads: usize,
    request_delay_ms: u64,
    download_delay_ms: u64,
}

fn ia_string_field(message: &Value, first: &str, second: &str) -> Option<String> {
    message[first]
        .as_str()
        .or_else(|| message[second].as_str())
        .map(std::string::ToString::to_string)
}

fn ia_usize_field(message: &Value, first: &str, second: &str) -> Option<usize> {
    message[first]
        .as_u64()
        .or_else(|| message[second].as_u64())
        .and_then(|v| usize::try_from(v).ok())
}

fn ia_u64_field(message: &Value, first: &str, second: &str) -> Option<u64> {
    message[first].as_u64().or_else(|| message[second].as_u64())
}

fn ia_config_from_message(message: &Value) -> IAMovieConfig {
    let output_dir = ia_string_field(message, "Local Save Path", "Output Dir")
        .unwrap_or_else(|| "/mediakraken/media/movie/internet_archive".to_string());
    let state_file = ia_string_field(message, "State File", "State Path").unwrap_or_else(|| {
        PathBuf::from(&output_dir)
            .join("downloaded_movies_state.json")
            .to_string_lossy()
            .to_string()
    });

    IAMovieConfig {
        query: ia_string_field(message, "Query", "Data")
            .unwrap_or_else(|| IA_DEFAULT_QUERY.to_string()),
        output_dir,
        state_file,
        max_items: ia_usize_field(message, "Max Items", "MaxItems").unwrap_or(IA_DEFAULT_MAX_ITEMS),
        max_downloads: ia_usize_field(message, "Max Downloads", "MaxDownloads")
            .unwrap_or(IA_DEFAULT_MAX_DOWNLOADS),
        request_delay_ms: ia_u64_field(message, "Request Delay MS", "RequestDelayMs")
            .unwrap_or(IA_DEFAULT_REQUEST_DELAY_MS),
        download_delay_ms: ia_u64_field(message, "Download Delay MS", "DownloadDelayMs")
            .unwrap_or(IA_DEFAULT_DOWNLOAD_DELAY_MS),
    }
}

fn ia_build_client() -> Result<reqwest::Client, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("MediaKraken-mkdownload/0.1 (+https://archive.org)"),
    );

    Ok(reqwest::Client::builder()
        .default_headers(headers)
        .build()?)
}

async fn ia_load_state(path: &str) -> Result<IAMovieState, Box<dyn Error>> {
    if !Path::new(path).exists() {
        return Ok(IAMovieState::default());
    }

    let content = fs::read_to_string(path).await?;
    Ok(serde_json::from_str::<IAMovieState>(&content)?)
}

async fn ia_save_state(path: &str, state: &IAMovieState) -> Result<(), Box<dyn Error>> {
    let content = serde_json::to_string_pretty(state)?;
    fs::write(path, content).await?;
    Ok(())
}

async fn ia_search_identifiers(
    client: &reqwest::Client,
    cfg: &IAMovieConfig,
) -> Result<Vec<String>, Box<dyn Error>> {
    let rows = cfg.max_items.to_string();
    let params = [
        ("q", cfg.query.as_str()),
        ("fl[]", "identifier"),
        ("sort[]", "downloads desc"),
        ("rows", rows.as_str()),
        ("page", "1"),
        ("output", "json"),
    ];

    let response = client
        .get(IA_ADVANCED_SEARCH_URL)
        .query(&params)
        .send()
        .await?
        .error_for_status()?;

    let body = response.json::<IASearchResponse>().await?;
    Ok(body
        .response
        .docs
        .into_iter()
        .map(|doc| doc.identifier)
        .collect())
}

fn ia_is_supported_format(format: &str) -> bool {
    let normalized = format.to_ascii_lowercase();
    normalized.contains("mpeg4") || normalized.contains("h.264") || normalized.contains("matroska")
}

async fn ia_select_download_url(
    client: &reqwest::Client,
    identifier: &str,
) -> Result<Option<String>, Box<dyn Error>> {
    let metadata_url = format!("{IA_METADATA_URL_PREFIX}{identifier}");
    let response = client.get(metadata_url).send().await?.error_for_status()?;
    let metadata = response.json::<IAMetadataResponse>().await?;

    let Some(files) = metadata.files else {
        return Ok(None);
    };

    let selected = files
        .iter()
        .filter(|file| {
            let name = file.name.to_ascii_lowercase();
            name.ends_with(".mp4") || name.ends_with(".mkv") || name.ends_with(".ogv")
        })
        .find(|file| file.format.as_deref().is_some_and(ia_is_supported_format))
        .map(|file| format!("https://archive.org/download/{identifier}/{}", file.name));

    Ok(selected)
}

async fn ia_download_to_file(
    client: &reqwest::Client,
    url: &str,
    output_file: &Path,
) -> Result<(), Box<dyn Error>> {
    let mut response = client.get(url).send().await?.error_for_status()?;
    let mut file = fs::File::create(output_file).await?;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
    }
    file.flush().await?;

    Ok(())
}

fn ia_safe_filename(identifier: &str, url: &str) -> String {
    let name = url.rsplit('/').next().unwrap_or("movie.bin");
    format!("{identifier}_{name}")
}

async fn handle_ia_movie(message: &Value) -> Result<(), Box<dyn Error>> {
    let cfg = ia_config_from_message(message);
    fs::create_dir_all(&cfg.output_dir).await?;

    let client = ia_build_client()?;
    let mut state = ia_load_state(&cfg.state_file).await?;
    let identifiers = ia_search_identifiers(&client, &cfg).await?;

    let mut downloaded_count = 0usize;
    for identifier in identifiers {
        if downloaded_count >= cfg.max_downloads {
            break;
        }
        if state.downloaded.contains(&identifier) {
            continue;
        }

        sleep(Duration::from_millis(cfg.request_delay_ms)).await;

        let Some(download_url) = ia_select_download_url(&client, &identifier).await? else {
            continue;
        };

        let file_name = ia_safe_filename(&identifier, &download_url);
        let output_file = PathBuf::from(&cfg.output_dir).join(file_name);
        ia_download_to_file(&client, &download_url, &output_file).await?;

        state.downloaded.insert(identifier);
        ia_save_state(&cfg.state_file, &state).await?;

        downloaded_count += 1;
        sleep(Duration::from_millis(cfg.download_delay_ms)).await;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect to db and do a version check
    let (_sqlx_pool_rw, sqlx_pool_ro) =
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

    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(payload) = msg.content {
                if let Ok(json_message) =
                    serde_json::from_str::<Value>(&String::from_utf8_lossy(&payload))
                {
                    let message_type = json_message["Type"].as_str().unwrap_or("");
                    if message_type == "File" {
                        if let (Some(url), Some(local_save_path)) = (
                            json_message["URL"].as_str(),
                            json_message["Local Save Path"].as_str(),
                        ) {
                            let _res = mk_lib_network::mk_lib_network::mk_download_file_from_url(
                                url.to_string(),
                                local_save_path,
                            )
                            .await;
                        }
                    } else if message_type == "Youtube" {
                        if validator::ValidateUrl::validate_url(
                            json_message["URL"].as_str().unwrap_or(""),
                        ) {
                            continue;
                        } else {
                            continue;
                        }
                    } else if message_type == "Subtitle" {
                        if let Some(data) = json_message["Data"].as_str() {
                            let _output = Command::new("subliminal")
                                .args(["-l", "en", data])
                                .stdout(Stdio::piped())
                                .output();
                        }
                    } else if message_type == "Twitch" {
                        if let (Some(url), Some(local_save_path)) = (
                            json_message["URL"].as_str(),
                            json_message["Local Save Path"].as_str(),
                        ) {
                            if validator::ValidateUrl::validate_url(url) {
                                let _res =
                                    mk_lib_network::mk_lib_network::mk_download_file_from_url_tokio(
                                        url.to_string(),
                                        local_save_path,
                                    )
                                    .await;
                            }
                        }
                    } else if message_type == "IAMovie" {
                        let _res = handle_ia_movie(&json_message).await;
                    // } else if message_type == "DigitalUPCNet" {
                    //     let sheet_data = mk_lib_metadata::mk_lib_metadata_provider_google_sheets::provider_google_sheets_fetch(
                    //         "1po70GCN9JUwrWgycMueNfxpEvBjLd7DQkiMRUQFFsL8".to_string(),
                    //         "tsv".to_string(),
                    //     )
                    //     .await
                    //     .unwrap();
                    // } else if message_type == "UPCMasterList" {
                    //     let sheet_data = mk_lib_metadata::mk_lib_metadata_provider_google_sheets::provider_google_sheets_fetch(
                    //         "1IgK7tIEKngP59PUOs_lsF4P1hbSRIG71tgxncpu1Mws".to_string(),
                    //         "tsv".to_string(),
                    //     )
                    //     .await
                    //     .unwrap();
                    } else if message_type == "Dosage" {
                        if let Some(data) = json_message["Data"].as_str() {
                            let _output = Command::new("dosage")
                                .args(["--adult", data])
                                .stdout(Stdio::piped())
                                .output();
                            if data == "--list" {
                                // TODO parse list and store the strips, see notes in data example
                            }
                        }
                    } else if message_type == "HDTrailers" {
                        if let Ok(feed_data) = mk_lib_network::mk_lib_network::mk_data_from_url(
                            "http://feeds.hd-trailers.net/hd-trailers".to_string(),
                        )
                        .await
                        {
                            if let Ok(data) = serde_json::from_str::<serde_json::Value>(&feed_data)
                            {
                                if let Some(an_array) = data["rss"]["channel"]["item"].as_array() {
                                    for item in an_array {
                                        if (item["title"].to_string().contains("(Trailer")
                                            && option_config_json["Metadata"]["Trailer"]["Trailer"]
                                                == true)
                                            || (item["title"].to_string().contains("(Behind")
                                                && option_config_json["Metadata"]["Trailer"]["Behind"]
                                                    == true)
                                            || (item["title"].to_string().contains("(Clip")
                                                && option_config_json["Metadata"]["Trailer"]["Clip"]
                                                    == true)
                                            || (item["title"].to_string().contains("(Featurette")
                                                && option_config_json["Metadata"]["Trailer"]["Featurette"]
                                                    == true)
                                            || (item["title"].to_string().contains("(Carpool")
                                                && option_config_json["Metadata"]["Trailer"]["Carpool"]
                                                    == true)
                                        {
                                            let download_link =
                                                item["enclosure"]["@url"].to_string();
                                            let file_save_name = format!(
                                                "/mediakraken/metadata/meta/trailer/{:?}",
                                                download_link.rsplitn(1, "/")
                                            );
                                            if !Path::new(&file_save_name).exists() {
                                                let _res = mk_lib_network::mk_lib_network::mk_download_file_from_url(
                                                    download_link.to_string(),
                                                    &file_save_name,
                                                )
                                                .await;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                let _result = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                    &rabbit_channel,
                    msg.deliver.unwrap().delivery_tag(),
                )
                .await;
            }
        }
    });

    let guard = Notify::new();
    guard.notified().await;
    Ok(())
}
