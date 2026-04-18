use reqwest::Client;

use crate::models::LibrarySummary;

#[derive(Clone, Debug)]
pub struct ApiClient {
    base_url: String,
    bearer_token: String,
    http_client: Client,
}

impl ApiClient {
    pub fn new(base_url: impl Into<String>, bearer_token: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_owned(),
            bearer_token: bearer_token.into(),
            http_client: Client::new(),
        }
    }

    pub async fn fetch_library_summary(&self) -> Result<LibrarySummary, reqwest::Error> {
        let mut request = self
            .http_client
            .get(format!("{}/api/v1/library/summary", self.base_url));

        if !self.bearer_token.trim().is_empty() {
            request = request.bearer_auth(self.bearer_token.trim());
        }

        request
            .send()
            .await?
            .error_for_status()?
            .json::<LibrarySummary>()
            .await
    }
}
