use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::time::{Duration, sleep};

type AppResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

const ADVANCED_SEARCH_URL: &str = "https://archive.org/advancedsearch.php";
const METADATA_URL_PREFIX: &str = "https://archive.org/metadata/";
const DEFAULT_QUERY: &str = "mediatype:(movies) AND collection:(feature_films)";

#[derive(Debug)]
struct Config {
    output_dir: String,
    state_file: String,
    max_items: usize,
    max_downloads: usize,
    request_delay_ms: u64,
    download_delay_ms: u64,
    query: String,
}

impl Config {
    fn from_env() -> Self {
        let output_dir = env::var("MK_OUTPUT_DIR").unwrap_or_else(|_| "downloads".to_string());
        let state_file = env::var("MK_STATE_FILE")
            .unwrap_or_else(|_| "downloaded_movies_state.json".to_string());
        let max_items = env::var("MK_MAX_ITEMS")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(30);
        let max_downloads = env::var("MK_MAX_DOWNLOADS")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(5);
        let request_delay_ms = env::var("MK_REQUEST_DELAY_MS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(1_000);
        let download_delay_ms = env::var("MK_DOWNLOAD_DELAY_MS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(2_000);
        let query = env::var("MK_QUERY").unwrap_or_else(|_| DEFAULT_QUERY.to_string());

        Self {
            output_dir,
            state_file,
            max_items,
            max_downloads,
            request_delay_ms,
            download_delay_ms,
            query,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct DownloadState {
    downloaded: HashSet<String>,
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    response: SearchDocs,
}

#[derive(Debug, Deserialize)]
struct SearchDocs {
    docs: Vec<SearchDoc>,
}

#[derive(Debug, Deserialize)]
struct SearchDoc {
    identifier: String,
}

#[derive(Debug, Deserialize)]
struct MetadataResponse {
    files: Option<Vec<ArchiveFile>>,
}

#[derive(Debug, Deserialize)]
struct ArchiveFile {
    name: String,
    format: Option<String>,
}

fn build_client() -> AppResult<reqwest::Client> {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("MediaKraken-ArchiveDownloader/0.1 (+https://archive.org)"),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    Ok(client)
}

async fn load_state(path: &str) -> AppResult<DownloadState> {
    if !Path::new(path).exists() {
        return Ok(DownloadState::default());
    }

    let content = fs::read_to_string(path).await?;
    let state = serde_json::from_str::<DownloadState>(&content)?;
    Ok(state)
}

async fn save_state(path: &str, state: &DownloadState) -> AppResult<()> {
    let content = serde_json::to_string_pretty(state)?;
    fs::write(path, content).await?;
    Ok(())
}

async fn search_items(client: &reqwest::Client, cfg: &Config) -> AppResult<Vec<String>> {
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
        .get(ADVANCED_SEARCH_URL)
        .query(&params)
        .send()
        .await?
        .error_for_status()?;

    let body = response.json::<SearchResponse>().await?;
    let ids = body
        .response
        .docs
        .into_iter()
        .map(|doc| doc.identifier)
        .collect::<Vec<_>>();
    Ok(ids)
}

fn supported_format(format: &str) -> bool {
    let normalized = format.to_ascii_lowercase();
    normalized.contains("mpeg4") || normalized.contains("h.264") || normalized.contains("matroska")
}

async fn select_download_url(
    client: &reqwest::Client,
    identifier: &str,
) -> AppResult<Option<String>> {
    let url = format!("{METADATA_URL_PREFIX}{identifier}");
    let response = client.get(url).send().await?.error_for_status()?;
    let metadata = response.json::<MetadataResponse>().await?;

    let Some(files) = metadata.files else {
        return Ok(None);
    };

    let selected = files
        .iter()
        .filter(|f| {
            let name = f.name.to_ascii_lowercase();
            name.ends_with(".mp4") || name.ends_with(".mkv") || name.ends_with(".ogv")
        })
        .find(|f| f.format.as_deref().is_some_and(supported_format))
        .map(|f| format!("https://archive.org/download/{identifier}/{}", f.name));

    Ok(selected)
}

async fn download_to_file(client: &reqwest::Client, url: &str, output_file: &str) -> AppResult<()> {
    let mut response = client.get(url).send().await?.error_for_status()?;
    let mut file = fs::File::create(output_file).await?;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
    }
    file.flush().await?;

    Ok(())
}

fn safe_filename(identifier: &str, url: &str) -> String {
    let name = url.rsplit('/').next().unwrap_or("movie.bin");
    format!("{identifier}_{name}")
}

#[tokio::main]
async fn main() -> AppResult<()> {
    let cfg = Config::from_env();
    fs::create_dir_all(&cfg.output_dir).await?;

    let client = build_client()?;
    let mut state = load_state(&cfg.state_file).await?;

    let identifiers = search_items(&client, &cfg).await?;
    let mut downloaded_count = 0usize;

    for identifier in identifiers {
        if downloaded_count >= cfg.max_downloads {
            break;
        }

        if state.downloaded.contains(&identifier) {
            continue;
        }

        sleep(Duration::from_millis(cfg.request_delay_ms)).await;

        let Some(download_url) = select_download_url(&client, &identifier).await? else {
            continue;
        };

        let filename = safe_filename(&identifier, &download_url);
        let output_path = format!("{}/{}", cfg.output_dir, filename);

        println!("Downloading {identifier} -> {output_path}");
        download_to_file(&client, &download_url, &output_path).await?;

        state.downloaded.insert(identifier.clone());
        save_state(&cfg.state_file, &state).await?;

        downloaded_count += 1;
        sleep(Duration::from_millis(cfg.download_delay_ms)).await;
    }

    println!(
        "Done. New downloads: {downloaded_count}. Known downloads tracked: {}",
        state.downloaded.len()
    );

    Ok(())
}
