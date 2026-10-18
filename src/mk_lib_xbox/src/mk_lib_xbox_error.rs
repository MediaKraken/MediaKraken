use thiserror::Error;

pub type Result<T> = std::result::Result<T, XboxApiError>;

#[derive(Debug, Error)]
pub enum XboxApiError {
    #[error("xbox live returned an error: {0}")]
    Api(String),

    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("xbox error: {0}")]
    Xbox(#[from] xbox::XboxError),
}
