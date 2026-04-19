use std::time::Duration;

use reqwest::Client;

use crate::models::LibrarySummary;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Clone, Debug)]
pub struct ApiClient {
    base_url: String,
    bearer_token: String,
    http_client: Client,
}

impl ApiClient {
    pub fn default_http_client() -> Client {
        Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .build()
            .expect("failed to build reqwest client")
    }

    pub fn from_parts(
        http_client: Client,
        base_url: impl Into<String>,
        bearer_token: impl Into<String>,
    ) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_owned(),
            bearer_token: bearer_token.into(),
            http_client,
        }
    }

    pub async fn fetch_library_summary(&self) -> Result<LibrarySummary, String> {
        let mut request = self
            .http_client
            .get(format!("{}/api/v1/library/summary", self.base_url));

        let token = self.bearer_token.trim();
        if !token.is_empty() {
            request = request.bearer_auth(token);
        }

        let response = request
            .send()
            .await
            .map_err(|err| format!("network error: {err}"))?
            .error_for_status()
            .map_err(|err| match err.status() {
                Some(status) => format!("server returned HTTP {status}"),
                None => format!("server error: {err}"),
            })?;

        response
            .json::<LibrarySummary>()
            .await
            .map_err(|err| format!("failed to decode response: {err}"))
    }
}

#[cfg(test)]
impl ApiClient {
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn client(base_url: &str) -> ApiClient {
        ApiClient::from_parts(ApiClient::default_http_client(), base_url, "")
    }

    #[test]
    fn trims_single_trailing_slash() {
        assert_eq!(client("https://example.com/").base_url(), "https://example.com");
    }

    #[test]
    fn trims_multiple_trailing_slashes() {
        assert_eq!(client("https://example.com///").base_url(), "https://example.com");
    }

    #[test]
    fn leaves_clean_url_unchanged() {
        assert_eq!(client("https://example.com").base_url(), "https://example.com");
    }
}
