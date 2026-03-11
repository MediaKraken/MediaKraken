use mk_lib_database;
use mk_lib_network;
use mk_lib_rabbitmq;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::sync::Notify;
use tokio::time::sleep;

const IA_ADVANCED_SEARCH_URL: &str = "https://archive.org/advancedsearch.php";
const IA_METADATA_URL_PREFIX: &str = "https://archive.org/metadata/";
const IA_DEFAULT_QUERY: &str = "mediatype:(movies) AND collection:(feature_films)";

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

async fn ia_load_state(state_file: &str) -> Result<IAMovieState, Box<dyn Error>> {
    if !Path::new(state_file).exists() {
        return Ok(IAMovieState::default());
    }
    let content = tokio::fs::read_to_string(state_file).await?;
    let state = serde_json::from_str::<IAMovieState>(&content)?;
    Ok(state)
}

async fn ia_save_state(state_file: &str, state: &IAMovieState) -> Result<(), Box<dyn Error>> {
    let data = serde_json::to_string_pretty(state)?;
    tokio::fs::write(state_file, data).await?;
    Ok(())
}

fn ia_supported_format(format_name: &str) -> bool {
    let normalized = format_name.to_ascii_lowercase();
    normalized.contains("mpeg4") || normalized.contains("h.264") || normalized.contains("matroska")
}

async fn ia_process_message(json_message: &Value) -> Result<(), Box<dyn Error>> {
    let output_dir = json_message["Local Save Path"]
        .as_str()
        .unwrap_or("/mediakraken/media/movie/internet_archive");
    tokio::fs::create_dir_all(output_dir).await?;

    let state_file = json_message["State File"].as_str().map_or_else(
        || format!("{}/downloaded_movies_state.json", output_dir),
        std::string::ToString::to_string,
    );
    let query = json_message["Query"]
        .as_str()
        .unwrap_or(IA_DEFAULT_QUERY)
        .to_string();
    let max_items = json_message["Max Items"].as_u64().unwrap_or(30) as usize;
    let max_downloads = json_message["Max Downloads"].as_u64().unwrap_or(5) as usize;
    let request_delay_ms = json_message["Request Delay MS"].as_u64().unwrap_or(1000);
    let download_delay_ms = json_message["Download Delay MS"].as_u64().unwrap_or(2000);

    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("MediaKraken-mkdownload/0.1 (+https://archive.org)"),
    );
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let row_count = max_items.to_string();
    let params = [
        ("q", query.as_str()),
        ("fl[]", "identifier"),
        ("sort[]", "downloads desc"),
        ("rows", row_count.as_str()),
        ("page", "1"),
        ("output", "json"),
    ];

    let search = client
        .get(IA_ADVANCED_SEARCH_URL)
        .query(&params)
        .send()
        .await?
        .error_for_status()?
        .json::<IASearchResponse>()
        .await?;

    let mut state = ia_load_state(&state_file).await?;
    let mut downloaded_count = 0usize;

    for item in search.response.docs {
        if downloaded_count >= max_downloads {
            break;
        }
        if state.downloaded.contains(&item.identifier) {
            continue;
        }

        sleep(Duration::from_millis(request_delay_ms)).await;

        let metadata_url = format!("{}{}", IA_METADATA_URL_PREFIX, item.identifier);
        let metadata = client
            .get(metadata_url)
            .send()
            .await?
            .error_for_status()?
            .json::<IAMetadataResponse>()
            .await?;

        let mut selected_url: Option<String> = None;
        if let Some(files) = metadata.files {
            for file in files {
                let lower_name = file.name.to_ascii_lowercase();
                let is_video = lower_name.ends_with(".mp4")
                    || lower_name.ends_with(".mkv")
                    || lower_name.ends_with(".ogv");
                let format_ok = file.format.as_deref().is_some_and(ia_supported_format);
                if is_video && format_ok {
                    selected_url = Some(format!(
                        "https://archive.org/download/{}/{}",
                        item.identifier, file.name
                    ));
                    break;
                }
            }
        }

        let Some(download_url) = selected_url else {
            continue;
        };

        let default_name = "movie.bin".to_string();
        let file_part = download_url.rsplit('/').next().unwrap_or(&default_name);
        let output_path = format!("{}/{}_{}", output_dir, item.identifier, file_part);

        let mut response = client.get(&download_url).send().await?.error_for_status()?;
        let mut out_file = tokio::fs::File::create(&output_path).await?;
        while let Some(chunk) = response.chunk().await? {
            out_file.write_all(&chunk).await?;
        }
        out_file.flush().await?;

        state.downloaded.insert(item.identifier);
        ia_save_state(&state_file, &state).await?;
        downloaded_count += 1;

        sleep(Duration::from_millis(download_delay_ms)).await;
    }

    Ok(())
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

    tokio::spawn(async move {
        while let Some(msg) = rabbit_consumer.recv().await {
            if let Some(payload) = msg.content {
                let json_message: Value =
                    serde_json::from_str(&String::from_utf8_lossy(&payload)).unwrap();
                //println!(" [x] Received {:?}", std::str::from_utf8(&payload).unwrap());
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
                        continue;
                        //println!("downloaded video to {:?}", rustube::download_best_quality(&json_message["URL"].to_string()).await.unwrap());
                    } else {
                        // TODO log error by user requested
                        continue;
                    }
                } else if json_message["Type"].to_string() == "Subtitle" {
                    let output = Command::new("subliminal")
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
                    } else {
                        // TODO log error by user requested
                        continue;
                    }
                } else if json_message["Type"].to_string() == "IAMovie" {
                    let _res = ia_process_message(&json_message).await;
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
                    let stdout = String::from_utf8(output.stdout).unwrap();
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
