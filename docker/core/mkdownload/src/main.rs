use mk_lib_database;
use mk_lib_network;
use mk_lib_rabbitmq;
use reqwest::{Client, StatusCode};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::env;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, OnceLock, RwLock};
use tokio::io::AsyncWriteExt;
use tokio::sync::Semaphore;
use tokio::time::{Duration, sleep};
use validator::ValidateUrl;

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
// Restrict all user-supplied write paths to this prefix to prevent path traversal.
const ALLOWED_WRITE_ROOT: &str = "/mediakraken/";
static DOSAGE_STRIP_CACHE: OnceLock<RwLock<Vec<String>>> = OnceLock::new();

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

fn sanitize_local_save_path(raw: &str) -> Result<PathBuf, TaskError> {
    if raw.is_empty() {
        return Err("Local Save Path is empty".to_string());
    }

    let path = PathBuf::from(raw);
    for component in path.components() {
        if matches!(component, Component::ParentDir) {
            return Err(format!("Local Save Path contains '..': {raw}"));
        }
    }

    if !path.starts_with(ALLOWED_WRITE_ROOT) {
        return Err(format!(
            "Local Save Path must be within {ALLOWED_WRITE_ROOT}: {raw}"
        ));
    }

    // Defense in depth: walk up to the deepest existing ancestor and
    // canonicalize it so a symlink under the allowed root can't redirect
    // writes outside it (e.g. /mediakraken/out -> /etc).
    let mut existing = path.as_path();
    while !existing.exists() {
        match existing.parent() {
            Some(parent) if !parent.as_os_str().is_empty() => existing = parent,
            _ => break,
        }
    }
    let canonical = std::fs::canonicalize(existing)
        .map_err(|e| format!("failed to canonicalize {}: {e}", existing.display()))?;
    if !canonical.starts_with(ALLOWED_WRITE_ROOT) {
        return Err(format!(
            "Local Save Path resolves outside {ALLOWED_WRITE_ROOT}: {raw} -> {}",
            canonical.display()
        ));
    }

    Ok(path)
}

async fn process_message(json_message: Value) -> Result<(), TaskError> {
    let message_type = json_message["Type"]
        .as_str()
        .ok_or("message missing Type")?;

    match message_type {
        "File" => {
            let url = json_message["URL"]
                .as_str()
                .ok_or("File message missing URL")?;
            let local_save_path = json_message["Local Save Path"]
                .as_str()
                .ok_or("File message missing Local Save Path")?;
            let local_save_path = sanitize_local_save_path(local_save_path)?;

            mk_lib_network::mk_lib_network::mk_download_file_from_url(
                url.to_string(),
                &local_save_path.to_string_lossy().into_owned(),
            )
            .await
            .map_err(|e| format!("file download failed: {e}"))?;
        }
        "Youtube" => {
            let url = json_message["URL"]
                .as_str()
                .ok_or("Youtube message missing URL")?;
            if !url.validate_url() {
                return Err("Youtube message contains an invalid URL".to_string());
            }

            let mut yt_dlp_command = Command::new("/yt-dlp");
            yt_dlp_command.arg("--no-progress");

            if let Some(raw) = json_message["Local Save Path"].as_str() {
                let local_save_path = sanitize_local_save_path(raw)?;
                yt_dlp_command.args(["-o", &local_save_path.to_string_lossy()]);
            }

            let output = yt_dlp_command
                .arg(url)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .map_err(|e| format!("youtube download command failed: {e}"))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
                return Err(format!("youtube download failed: {stderr}"));
            }
        }
        "Subtitle" => {
            let data = json_message["Data"]
                .as_str()
                .ok_or("Subtitle message missing Data")?;

            let output = Command::new("subliminal")
                .args(["-l", "en", data])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .map_err(|e| format!("subtitle download command failed: {e}"))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
                return Err(format!("subtitle download failed: {stderr}"));
            }
        }
        "Twitch" => {
            let url = json_message["URL"]
                .as_str()
                .ok_or("Twitch message missing URL")?;
            let local_save_path = json_message["Local Save Path"]
                .as_str()
                .ok_or("Twitch message missing Local Save Path")?;

            if !url.validate_url() {
                return Err("Twitch message contains an invalid URL".to_string());
            }

            let local_save_path = sanitize_local_save_path(local_save_path)?;
            mk_lib_network::mk_lib_network::mk_download_file_from_url_tokio(
                url.to_string(),
                &local_save_path.to_string_lossy(),
            )
            .await
            .map_err(|e| format!("twitch download failed: {e}"))?;
        }
        "Dosage" => {
            let data = json_message["Data"]
                .as_str()
                .ok_or("Dosage message missing Data")?;

            if data == "--all" {
                let output_root = json_message["Local Save Path"]
                    .as_str()
                    .unwrap_or("/mediakraken/metadata/meta/comics");
                let output_root = sanitize_local_save_path(output_root)?;
                dosage_download_all_strips(&output_root.to_string_lossy())?;
            } else if data == "--list" {
                let output = Command::new("dosage")
                    .args(["--adult", "--list"])
                    .stdout(Stdio::piped())
                    .output()
                    .map_err(|e| format!("dosage --list failed: {e}"))?;
                let stdout = String::from_utf8_lossy(&output.stdout);
                dosage_store_strips(dosage_parse_strip_list(&stdout));
            } else {
                // Only accept strip names that appear in the cached list to
                // block arbitrary flags being passed to dosage.
                if !dosage_is_known_strip(data) {
                    return Err(format!("Dosage message contains unknown strip: {data}"));
                }
                let status = Command::new("dosage")
                    .args(["--adult", data])
                    .stdout(Stdio::piped())
                    .status()
                    .map_err(|e| format!("dosage failed: {e}"))?;
                if !status.success() {
                    return Err(format!("dosage exited with status {status}"));
                }
            }
        }
        "HDTrailers" => {
            let download_links = hdtrailers_collect_best_links().await?;
            for download_link in download_links {
                if let Some(filename) = hdtrailers_extract_filename(&download_link) {
                    let file_save_name = format!("/mediakraken/metadata/meta/trailer/{filename}");

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
        "IATrailer" | "IAMovies" => {
            process_ia_trailer_request(&json_message).await?;
        }
        other => {
            return Err(format!("unknown message type: {other}"));
        }
    }

    Ok(())
}

fn dosage_parse_strip_list(list_output: &str) -> Vec<String> {
    let mut strips = Vec::new();

    for raw_line in list_output.lines() {
        let line = raw_line.trim_end();
        if line.is_empty()
            || line.starts_with("Available comic scrapers:")
            || line.starts_with("Comics tagged with ")
            || line.starts_with("Non-english comics ")
            || line.starts_with("Some comics are disabled")
            || line.ends_with("supported comics.")
            || line.starts_with("  ")
            || line.starts_with('*')
        {
            continue;
        }

        let mut segment_start = 0_usize;
        let bytes = line.as_bytes();
        let mut idx = 0_usize;

        while idx < bytes.len() {
            if bytes[idx] == b' ' {
                let gap_start = idx;
                while idx < bytes.len() && bytes[idx] == b' ' {
                    idx += 1;
                }

                if idx - gap_start >= 2 {
                    let item = line[segment_start..gap_start].trim();
                    if !item.is_empty() {
                        strips.push(item.to_string());
                    }
                    segment_start = idx;
                }
                continue;
            }

            idx += 1;
        }

        let item = line[segment_start..].trim();
        if !item.is_empty() {
            strips.push(item.to_string());
        }
    }

    strips
}

fn dosage_store_strips(strips: Vec<String>) {
    if strips.is_empty() {
        return;
    }

    let strip_cache = DOSAGE_STRIP_CACHE.get_or_init(|| RwLock::new(Vec::new()));
    if let Ok(mut cache) = strip_cache.write() {
        *cache = strips;
    } else {
        eprintln!("dosage strip cache lock poisoned");
    }
}

fn dosage_get_cached_strips() -> Vec<String> {
    let Some(strip_cache) = DOSAGE_STRIP_CACHE.get() else {
        return Vec::new();
    };

    match strip_cache.read() {
        Ok(cache) => cache.clone(),
        Err(_) => {
            eprintln!("dosage strip cache lock poisoned");
            Vec::new()
        }
    }
}

fn dosage_is_known_strip(name: &str) -> bool {
    let mut strips = dosage_get_cached_strips();
    if strips.is_empty() {
        strips = dosage_fetch_strip_list().unwrap_or_default();
    }
    strips.iter().any(|s| s == name)
}

fn dosage_fetch_strip_list() -> Result<Vec<String>, TaskError> {
    let output = Command::new("dosage")
        .args(["--adult", "--list"])
        .stdout(Stdio::piped())
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(format!(
            "dosage --list failed with status {}",
            output.status
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let strips = dosage_parse_strip_list(&stdout);
    dosage_store_strips(strips.clone());
    Ok(strips)
}

fn dosage_strip_output_dir(root: &Path, strip: &str) -> PathBuf {
    let mut output = root.to_path_buf();

    for segment in strip.split('/') {
        let cleaned = match segment {
            "" | "." | ".." => "_",
            _ => segment,
        };
        output.push(cleaned);
    }

    output
}

fn dosage_download_all_strips(output_root: &str) -> Result<(), TaskError> {
    let output_root_path = Path::new(output_root);
    std::fs::create_dir_all(output_root_path).map_err(|e| e.to_string())?;

    let mut strips = dosage_get_cached_strips();
    if strips.is_empty() {
        strips = dosage_fetch_strip_list()?;
    }

    if strips.is_empty() {
        return Err("no dosage strips found".to_string());
    }

    for strip in strips {
        let strip_output_dir = dosage_strip_output_dir(output_root_path, &strip);
        std::fs::create_dir_all(&strip_output_dir).map_err(|e| e.to_string())?;

        let status = Command::new("dosage")
            .args([
                "--adult",
                "--all",
                "--basepath",
                strip_output_dir.to_string_lossy().as_ref(),
                strip.as_str(),
            ])
            .status()
            .map_err(|e| e.to_string())?;

        if !status.success() {
            eprintln!("dosage download failed for strip {strip}: {status}");
        }
    }

    Ok(())
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
    let without_fragment = url.split_once('#').map(|(left, _)| left).unwrap_or(url);
    let without_query = without_fragment
        .split_once('?')
        .map(|(left, _)| left)
        .unwrap_or(without_fragment);
    let file_name = without_query.rsplit('/').next()?;
    if file_name.is_empty() {
        return None;
    }
    Some(file_name.to_string())
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let (_sqlx_pool_rw, sqlx_pool_ro) =
        mk_lib_database::mk_lib_database::mk_lib_database_open_pool(10, 120).await?;
    mk_lib_database::mk_lib_database_version::mk_lib_database_version_check(&sqlx_pool_ro, false)
        .await?;

    let _option_config_json: Value =
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
    let rabbit_channel = Arc::new(rabbit_channel);

    let consumer_task = tokio::spawn({
        let worker_limit = Arc::clone(&worker_limit);
        let rabbit_channel = Arc::clone(&rabbit_channel);

        async move {
            while let Some(msg) = rabbit_consumer.recv().await {
                let Some(payload) = msg.content else {
                    continue;
                };

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
                let rabbit_channel = Arc::clone(&rabbit_channel);

                tokio::spawn(async move {
                    let permit = match worker_limit.acquire_owned().await {
                        Ok(permit) => permit,
                        Err(error) => {
                            eprintln!("failed to acquire worker permit: {error}");
                            return;
                        }
                    };

                    if let Err(error) = process_message(json_message).await {
                        eprintln!("message processing failed: {error}");
                    }

                    if let Err(error) = mk_lib_rabbitmq::mk_lib_rabbitmq::rabbitmq_ack(
                        &rabbit_channel,
                        delivery_tag,
                    )
                    .await
                    {
                        eprintln!("rabbit ack failed: {error}");
                    }

                    drop(permit);
                });
            }
        }
    });

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            eprintln!("received ctrl-c, shutting down");
        }
        result = consumer_task => {
            if let Err(error) = result {
                eprintln!("consumer task ended unexpectedly: {error}");
            }
        }
    }

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

        let mut dirty = false;

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
                dirty = true;
                sleep(IA_DOWNLOAD_DELAY).await;
            }
        }

        // Flush once per page instead of per-doc.
        if dirty {
            ia_save_state(&state_file, &state).await?;
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
        let status = response.status();

        if status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error() {
            if attempt > IA_MAX_RETRIES {
                return Err(format!(
                    "request failed ({status}) after {IA_MAX_RETRIES} retries for {url}"
                ));
            }

            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or_else(|| 1 << (attempt - 1).min(5));

            sleep(Duration::from_secs(retry_after)).await;
            continue;
        }

        if !status.is_success() {
            return Err(format!("request failed ({status}): {url}"));
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
    // Write to a sibling temp file then rename so a crash mid-write can't
    // corrupt the existing state.
    let tmp_path = match path.extension() {
        Some(ext) => {
            let mut new_ext = ext.to_os_string();
            new_ext.push(".tmp");
            path.with_extension(new_ext)
        }
        None => path.with_extension("tmp"),
    };
    tokio::fs::write(&tmp_path, data)
        .await
        .map_err(|e| e.to_string())?;
    tokio::fs::rename(&tmp_path, path)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}
