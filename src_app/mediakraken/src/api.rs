use std::time::Duration;

use reqwest::Client;
use serde::Serialize;

use crate::models::{
    HardwareDevice, HardwareDeviceList, LibrarySummary, MediaKind, MediaList, UpcLookupResult,
};

const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Clone, Debug)]
pub struct ApiClient {
    base_url: String,
    bearer_token: String,
    http_client: Client,
}

impl PartialEq for ApiClient {
    fn eq(&self, other: &Self) -> bool {
        self.base_url == other.base_url && self.bearer_token == other.bearer_token
    }
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
        self.get_json("/api/v1/library/summary").await
    }

    pub async fn lookup_upc(&self, upc: &str) -> Result<UpcLookupResult, String> {
        let trimmed = upc.trim();
        if trimmed.is_empty() {
            return Err("enter a UPC code before scanning".to_string());
        }
        let encoded = urlencode(trimmed);
        self.get_json(&format!("/api/v1/library/ownership?upc={encoded}"))
            .await
    }

    pub async fn list_hardware(&self) -> Result<HardwareDeviceList, String> {
        self.get_json("/api/v1/hardware/devices").await
    }

    pub async fn set_device_volume(
        &self,
        device_id: &str,
        volume: u8,
    ) -> Result<HardwareDevice, String> {
        #[derive(Serialize)]
        struct VolumeBody {
            volume: u8,
        }
        self.post_json(
            &format!("/api/v1/hardware/devices/{}/volume", urlencode(device_id)),
            &VolumeBody { volume },
        )
        .await
    }

    pub async fn set_device_power(
        &self,
        device_id: &str,
        powered: bool,
    ) -> Result<HardwareDevice, String> {
        #[derive(Serialize)]
        struct PowerBody {
            powered: bool,
        }
        self.post_json(
            &format!("/api/v1/hardware/devices/{}/power", urlencode(device_id)),
            &PowerBody { powered },
        )
        .await
    }

    pub async fn set_device_mute(
        &self,
        device_id: &str,
        muted: bool,
    ) -> Result<HardwareDevice, String> {
        #[derive(Serialize)]
        struct MuteBody {
            muted: bool,
        }
        self.post_json(
            &format!("/api/v1/hardware/devices/{}/mute", urlencode(device_id)),
            &MuteBody { muted },
        )
        .await
    }

    pub async fn list_media(&self, kind: MediaKind) -> Result<MediaList, String> {
        self.get_json(&format!("/api/v1/media/{}", kind.endpoint()))
            .await
    }

    async fn get_json<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, String> {
        let mut request = self.http_client.get(format!("{}{path}", self.base_url));
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
            .json::<T>()
            .await
            .map_err(|err| format!("failed to decode response: {err}"))
    }

    async fn post_json<B: Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, String> {
        let mut request = self
            .http_client
            .post(format!("{}{path}", self.base_url))
            .json(body);
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
            .json::<T>()
            .await
            .map_err(|err| format!("failed to decode response: {err}"))
    }
}

fn urlencode(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for byte in value.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*byte as char);
            }
            other => {
                out.push_str(&format!("%{other:02X}"));
            }
        }
    }
    out
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
        assert_eq!(client("https://www.mediakraken.media/").base_url(), "https://www.mediakraken.media");
    }

    #[test]
    fn trims_multiple_trailing_slashes() {
        assert_eq!(client("https://www.mediakraken.media///").base_url(), "https://www.mediakraken.media");
    }

    #[test]
    fn leaves_clean_url_unchanged() {
        assert_eq!(client("https://www.mediakraken.media").base_url(), "https://www.mediakraken.media");
    }

    #[test]
    fn urlencode_passes_unreserved_chars() {
        assert_eq!(urlencode("ABC-abc_123.~"), "ABC-abc_123.~");
    }

    #[test]
    fn urlencode_encodes_reserved_chars() {
        assert_eq!(urlencode("a b/c?"), "a%20b%2Fc%3F");
    }
}
