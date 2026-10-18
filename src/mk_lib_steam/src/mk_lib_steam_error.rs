use thiserror::Error;

pub type Result<T> = std::result::Result<T, SteamError>;

#[derive(Debug, Error)]
pub enum SteamError {
    #[error("steam web api returned an error: {0}")]
    Api(String),

    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("rsteam error: {0}")]
    Rsteam(#[from] rsteam::error::Error),
}
