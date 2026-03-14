use mk_lib_database;
use mk_lib_hash;
use mk_lib_network;
use mk_lib_rabbitmq;
use reqwest::{Client, StatusCode};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::{Notify, Semaphore};
use tokio::time::{Duration, sleep};
use walkdir::{DirEntry, WalkDir};

type AppError = Box<dyn std::error::Error>;
type TaskError = String;

const DEFAULT_MAX_CONCURRENT_DOWNLOADS: usize = 4;
const IA_SEARCH_ROWS: usize = 25;
const IA_SEARCH_DELAY: Duration = Duration::from_secs(2);
const IA_DOWNLOAD_DELAY: Duration = Duration::from_secs(1);
const IA_MAX_RETRIES: usize = 3;
const IA_DEFAULT_OUTPUT_DIR: &str = "/mediakraken/metadata/meta/trailer/internet_archive";
const IA_DEFAULT_STATE_FILE: &str =
    "/mediakraken/metadata/meta/trailer/internet_archive_state.json";
const IA_DEFAULT_MAX_PAGES: usize = 1;

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
struct IATrailerState {
    downloaded_ids: HashSet<String>,
    downloaded_files: HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize)]
struct IASearchResponse {
    response: IASearchDocs,
}

#[derive(Debug, serde::Deserialize)]
struct IASearchDocs {
    docs: Vec<IASearchDoc>,
}

#[derive(Debug, serde::Deserialize)]
struct IASearchDoc {
    identifier: String,
}

#[derive(Debug, serde::Deserialize)]
struct IAMetadataResponse {
    files: Vec<IAArchiveFile>,
}

#[derive(Debug, serde::Deserialize)]
struct IAArchiveFile {
    name: Option<String>,
    format: Option<String>,
}

async fn process_message(json_message: Value, _option_config_json: &Value) {
    match json_message["Type"].as_str() {
        Some("File") => {
            if let (Some(url), Some(local_save_path)) = (
                json_message["URL"].as_str(),
                json_message["Local Save Path"].as_str(),
            ) {
                let local_save_path = local_save_path.to_string();

                if let Err(error) = mk_lib_network::mk_lib_network::mk_download_file_from_url(
                    url.to_string(),
                    &local_save_path,
                )
                .await
                {
                    eprintln!("file download failed: {error}");
                }
            } else {
                eprintln!("File message missing URL or Local Save Path");
            }
        }
        Some("Youtube") => {
            if let Some(url) = json_message["URL"].as_str() {
                if validator::ValidateUrl::validate_url(url) {
                    return;
                }
            }
        }
        Some("Subtitle") => {
            if let Some(data) = json_message["Data"].as_str() {
                let result = Command::new("subliminal")
                    .args(["-l", "en", data])
                    .stdout(Stdio::piped())
                    .output();

                if let Err(error) = result {
                    eprintln!("subtitle download failed: {error}");
                }
            } else {
                eprintln!("Subtitle message missing Data");
            }
        }
        Some("Twitch") => {
            if let (Some(url), Some(local_save_path)) = (
                json_message["URL"].as_str(),
                json_message["Local Save Path"].as_str(),
            ) {
                let local_save_path = local_save_path.to_string();

                if validator::ValidateUrl::validate_url(url) {
                    if let Err(error) =
                        mk_lib_network::mk_lib_network::mk_download_file_from_url_tokio(
                            url.to_string(),
                            &local_save_path,
                        )
                        .await
                    {
                        eprintln!("twitch download failed: {error}");
                    }
                }
            } else {
                eprintln!("Twitch message missing URL or Local Save Path");
            }
        }
        Some("Dosage") => {
            if let Some(data) = json_message["Data"].as_str() {
                let output = Command::new("dosage")
                    .args(["--adult", data])
                    .stdout(Stdio::piped())
                    .output();

                match output {
                    Ok(output) => {
                        let _stdout = String::from_utf8_lossy(&output.stdout);
                        if data == "--list" {
                            // TODO parse list and store the strips
                        }
                    }
                    Err(error) => eprintln!("dosage failed: {error}"),
                }
            } else {
                eprintln!("Dosage message missing Data");
            }
        }
        Some("HDTrailers") => match hdtrailers_collect_best_links().await {
            Ok(download_links) => {
                for download_link in download_links {
                    if let Some(filename) = hdtrailers_extract_filename(&download_link) {
                        let file_save_name =
                            format!("/mediakraken/metadata/meta/trailer/{filename}");

                        if !Path::new(&file_save_name).exists() {
                            if let Err(error) =
                                mk_lib_network::mk_lib_network::mk_download_file_from_url(
                                    download_link,
                                    &file_save_name,
                                )
                                .await
                            {
                                eprintln!("hdtrailers download failed: {error}");
                            }
                        }
                    }
                }
            }
            Err(error) => eprintln!("hdtrailers collect failed: {error}"),
        },
        Some("IATrailer") | Some("IAMovies") => {
            if let Err(error) = process_ia_trailer_request(&json_message).await {
                eprintln!(
                    "{} request failed: {error}",
                    json_message["Type"].as_str().unwrap_or("unknown")
                );
            }
        }
        Some("LibretroCoreFetchUpdate") | Some("mklibretrocorefetchupdate") => {
            if let Err(error) = process_libretro_core_fetch_update().await {
                eprintln!("libretro core update failed: {error}");
            }
        }
        Some(other) => {
            eprintln!("unknown message type: {other}");
        }
        None => {
            eprintln!("message missing Type");
        }
    }
}

async fn hdtrailers_collect_best_links() -> Result<Vec<String>, TaskError> {
    let homepage_html = mk_lib_network::mk_lib_network::mk_data_from_url(
        "https://www.hd-trailers.net/".to_string(),
    )
    .await
    .map_err(|e| e.to_string())?;

    let mut movie_pages = HashSet::new();
    for href in hdtrailers_extract_hrefs(&homepage_html) {
        if href.starts_with("https://www.hd-trailers.net/movie/") {
            movie_pages.insert(href);
        } else if href.starts_with("/movie/") {
            movie_pages.insert(format!("https://www.hd-trailers.net{href}"));
        }
    }

    let mut best_links = Vec::new();
    for movie_page in movie_pages {
        let page_html = match mk_lib_network::mk_lib_network::mk_data_from_url(movie_page).await {
            Ok(value) => value,
            Err(_) => continue,
        };

        if let Some(best) = hdtrailers_find_best_video_link(&page_html) {
            best_links.push(best);
        }
    }

    Ok(best_links)
}

fn hdtrailers_extract_hrefs(html: &str) -> Vec<String> {
    let mut hrefs = Vec::new();
    let mut offset = 0;

    while let Some(pos) = html[offset..].find("href=") {
        let start = offset + pos + 5;
        let Some(delimiter) = html[start..].chars().next() else {
            break;
        };
        if delimiter != '"' && delimiter != '\'' {
            offset = start;
            continue;
        }

        let value_start = start + delimiter.len_utf8();
        let Some(value_end_rel) = html[value_start..].find(delimiter) else {
            break;
        };
        let value_end = value_start + value_end_rel;
        hrefs.push(html[value_start..value_end].to_string());
        offset = value_end + delimiter.len_utf8();
    }

    hrefs
}

fn hdtrailers_find_best_video_link(html: &str) -> Option<String> {
    let mut best_link: Option<String> = None;
    let mut best_score = 0_u64;

    for href in hdtrailers_extract_hrefs(html) {
        let lower = href.to_ascii_lowercase();
        if !(lower.ends_with(".mp4")
            || lower.ends_with(".mov")
            || lower.ends_with(".m4v")
            || lower.ends_with(".webm"))
        {
            continue;
        }

        let score = hdtrailers_link_score(&lower);
        if score > best_score {
            best_score = score;
            best_link = Some(href);
        }
    }

    best_link
}

fn hdtrailers_link_score(link: &str) -> u64 {
    let mut score = 0_u64;

    for height in [4320_u64, 2160, 1080, 720, 576, 480, 360] {
        let token = format!("{height}p");
        if link.contains(&token) {
            score = score.max(height);
        }
    }

    for part in link.split(|value: char| !(value.is_ascii_alphanumeric() || value == 'x')) {
        let Some((left, right)) = part.split_once('x') else {
            continue;
        };
        if left.is_empty() || right.is_empty() {
            continue;
        }
        if !left.chars().all(|value| value.is_ascii_digit())
            || !right.chars().all(|value| value.is_ascii_digit())
        {
            continue;
        }

        if let (Ok(width), Ok(height)) = (left.parse::<u64>(), right.parse::<u64>()) {
            score = score.max(width.saturating_mul(height));
        }
    }

    score
}

fn hdtrailers_extract_filename(url: &str) -> Option<String> {
    let file_name = url.rsplit('/').next()?;
    if file_name.is_empty() {
        return None;
    }
    Some(file_name.to_string())
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
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
        .filter_map(|segment| segment.split('"').next())
        .filter_map(|href| href.strip_suffix('/'))
        .filter(|version| version.chars().all(|c| c.is_ascii_digit() || c == '.'))
        .filter_map(|version| parse_version_components(version).map(|parts| (parts, version)))
        .max_by(|(left, _), (right, _)| left.cmp(right))
        .map(|(_, version)| version.to_string())
}

async fn process_libretro_core_fetch_update() -> Result<(), TaskError> {
    let mut emulation_cores = HashMap::new();
    let walker = WalkDir::new("/mediakraken/emulation/cores").into_iter();
    for entry in walker
        .filter_entry(|e| !is_hidden(e))
        .filter_map(Result::ok)
        .filter(|d| d.path().extension() == Some(OsStr::from_bytes(b"zip")))
        .filter(|e| !e.file_type().is_dir())
    {
        let file_name = entry.path().display().to_string();
        let crc = mk_lib_hash::mk_lib_hash_crc32::mk_file_hash_crc32(&file_name)
            .await
            .map_err(|e| e.to_string())?;
        emulation_cores.insert(file_name, crc);
    }

    let stable_root = "http://buildbot.libretro.com/stable/";
    let stable_index = mk_lib_network::mk_lib_network::mk_data_from_url(stable_root.to_string())
        .await
        .map_err(|e| e.to_string())?;
    let latest_version = latest_stable_version(&stable_index)
        .ok_or_else(|| "unable to determine latest libretro stable version".to_string())?;
    let libtro_url = format!("{}{}/linux/x86_64/", stable_root, latest_version);
    let fetch_result = mk_lib_network::mk_lib_network::mk_data_from_url(format!(
        "{}{}",
        &libtro_url, ".index-extended"
    ))
    .await
    .map_err(|e| e.to_string())?;

    for libretro_core in fetch_result.lines().filter(|line| !line.is_empty()) {
        let mut iter = libretro_core.splitn(3, ' ');
        let Some(_core_date) = iter.next() else {
            continue;
        };
        let Some(core_crc32) = iter.next() else {
            continue;
        };
        let Some(core_name) = iter.next() else {
            continue;
        };

        let path_core_name = format!(
            "/mediakraken/emulation/cores/{}",
            core_name.replace(".zip", "")
        );

        let download_core = match emulation_cores.get(&path_core_name) {
            Some(existing_crc) => existing_crc != core_crc32,
            None => true,
        };

        if download_core {
            mk_lib_network::mk_lib_network::mk_download_file_from_url(
                format!("{}{}", &libtro_url, core_name),
                &format!("/mediakraken/emulation/cores/{}", core_name),
            )
            .await
            .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

fn sync_project_gutenberg(
    destination: &str,
    source: Option<&str>,
    dry_run: bool,
) -> Result<(), AppError> {
    if !Path::new(destination).exists() {
        return Err(format!("Destination does not exist: {destination}").into());
    }

    let rsync_source = source.unwrap_or("rsync://mirrors.xmission.com/gutenberg/");
    let mut command = Command::new("rsync");
    command.args(["-avz", "--delete"]);

    if dry_run {
        command.arg("--dry-run");
    }

    command.args([rsync_source, destination]);

    let status = command.status()?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("rsync failed with status: {status}").into())
    }
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let (_sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(50, 120).await?;
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
        .await?;

    let option_config_json: Value =
        mk_lib_database::mk_lib_database_option_status::mk_lib_database_option_read(&sqlx_pool_ro)
            .await?;

    let (_rabbit_connection, rabbit_channel) =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_connect("mkdownload").await?;

    let mut rabbit_consumer =
        mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_consumer("mkdownload", &rabbit_channel).await?;

    let max_concurrent_downloads = env::var("MKDOWNLOAD_CONCURRENT_WORKERS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_MAX_CONCURRENT_DOWNLOADS);

    let worker_limit = Arc::new(Semaphore::new(max_concurrent_downloads));
    let option_config_json = Arc::new(option_config_json);
    let rabbit_channel = Arc::new(rabbit_channel);

    tokio::spawn({
        let worker_limit = Arc::clone(&worker_limit);
        let option_config_json = Arc::clone(&option_config_json);
        let rabbit_channel = Arc::clone(&rabbit_channel);

        async move {
            while let Some(msg) = rabbit_consumer.recv().await {
                if let Some(payload) = msg.content {
                    let json_message: Value =
                        match serde_json::from_str(&String::from_utf8_lossy(&payload)) {
                            Ok(value) => value,
                            Err(error) => {
                                eprintln!("invalid JSON message: {error}");
                                continue;
                            }
                        };

                    let delivery_tag = match msg.deliver {
                        Some(deliver) => deliver.delivery_tag(),
                        None => {
                            eprintln!("rabbit message missing delivery metadata");
                            continue;
                        }
                    };

                    let worker_limit = Arc::clone(&worker_limit);
                    let option_config_json = Arc::clone(&option_config_json);
                    let rabbit_channel = Arc::clone(&rabbit_channel);

                    tokio::spawn(async move {
                        match worker_limit.acquire_owned().await {
                            Ok(_permit) => {
                                process_message(json_message, option_config_json.as_ref()).await;

                                if let Err(error) = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                                    &rabbit_channel,
                                    delivery_tag,
                                )
                                .await
                                {
                                    eprintln!("rabbit ack failed: {error}");
                                }
                            }
                            Err(error) => {
                                eprintln!("failed to acquire worker permit: {error}");
                            }
                        }
                    });
                }
            }
        }
    });

    let guard = Notify::new();
    guard.notified().await;
    Ok(())
}

async fn process_ia_trailer_request(json_message: &Value) -> Result<(), TaskError> {
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

    tokio::fs::create_dir_all(&output_dir)
        .await
        .map_err(|e| e.to_string())?;

    let mut state = ia_load_state(&state_file).await?;
    let client = Client::builder()
        .user_agent("mediakraken-mkdownload-ia-trailer/0.1 (+https://archive.org)")
        .build()
        .map_err(|e| e.to_string())?;

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
) -> Result<Vec<IASearchDoc>, TaskError> {
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
) -> Result<Option<String>, TaskError> {
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
) -> Result<PathBuf, TaskError> {
    let url = format!(
        "https://archive.org/download/{}/{}",
        urlencoding::encode(identifier),
        urlencoding::encode(filename)
    );

    let mut response = ia_get_with_backoff(client, &url).await?;
    let output_name = ia_sanitize_filename(identifier, filename);
    let output_path = output_dir.join(output_name);
    let mut file = tokio::fs::File::create(&output_path)
        .await
        .map_err(|e| e.to_string())?;

    while let Some(chunk) = response.chunk().await.map_err(|e| e.to_string())? {
        file.write_all(&chunk).await.map_err(|e| e.to_string())?;
    }
    file.flush().await.map_err(|e| e.to_string())?;

    Ok(output_path)
}

async fn ia_fetch_json_with_backoff<T>(client: &Client, url: &str) -> Result<T, TaskError>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let response = ia_get_with_backoff(client, url).await?;
    response.json::<T>().await.map_err(|e| e.to_string())
}

async fn ia_get_with_backoff(client: &Client, url: &str) -> Result<reqwest::Response, TaskError> {
    let mut attempt = 0;
    loop {
        attempt += 1;
        let response = client.get(url).send().await.map_err(|e| e.to_string())?;

        if response.status() == StatusCode::TOO_MANY_REQUESTS {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or(5);

            if attempt > IA_MAX_RETRIES {
                return Err(format!(
                    "rate limited after {IA_MAX_RETRIES} retries for {url}"
                ));
            }

            sleep(Duration::from_secs(retry_after)).await;
            continue;
        }

        if !response.status().is_success() {
            return Err(format!("request failed ({}): {url}", response.status()));
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

async fn ia_load_state(path: &Path) -> Result<IATrailerState, TaskError> {
    match tokio::fs::read(path).await {
        Ok(bytes) => serde_json::from_slice(&bytes).map_err(|e| e.to_string()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(IATrailerState::default()),
        Err(error) => Err(error.to_string()),
    }
}

async fn ia_save_state(path: &Path, state: &IATrailerState) -> Result<(), TaskError> {
    let data = serde_json::to_vec_pretty(state).map_err(|e| e.to_string())?;
    tokio::fs::write(path, data)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
