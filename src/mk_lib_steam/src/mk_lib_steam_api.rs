use std::ops::Deref;

use rsteam::SteamClient;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::mk_lib_steam_error::{Result, SteamError};

const STEAM_API_HOST: &str = "api.steampowered.com";

/// Entry point for all Steam Web API calls.
///
/// Derefs to [`SteamClient`], so all typed rsteam endpoints (ISteamUser,
/// ISteamUserStats, IPlayerService, ISteamApps, ISteamNews, legacy group
/// endpoints) are available directly, e.g.
/// `api.get_player_summaries(&ids).await`.
///
/// Any other Steam Web API endpoint can be called with [`SteamApi::call`].
pub struct SteamApi {
    api_key: Option<String>,
    http: reqwest::Client,
    client: SteamClient,
}

impl Default for SteamApi {
    fn default() -> Self {
        Self::new()
    }
}

impl SteamApi {
    /// Creates a client without an API key.
    ///
    /// Only keyless endpoints work (e.g. player counts, news, app list).
    pub fn new() -> Self {
        Self::build(None)
    }

    /// Creates a client with an API key.
    ///
    /// All endpoints work, including the keyless ones.
    pub fn with_api_key(key: &str) -> Self {
        Self::build(Some(key))
    }

    fn build(key: Option<&str>) -> Self {
        Self {
            api_key: key.map(|k| k.to_owned()),
            http: reqwest::Client::new(),
            client: match key {
                Some(k) => SteamClient::with_api_key(k),
                None => SteamClient::new(),
            },
        }
    }

    /// The API key this client was created with, if any.
    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
    }

    /// Calls any Steam Web API endpoint and returns the raw JSON body.
    ///
    /// `interface` is e.g. `"ISteamApps"`, `method` is e.g.
    /// `"GetAppDetails"`, `version` is e.g. `"v0001"`.
    /// Query parameters are url-encoded and appended; the API key is added
    /// automatically when the client has one.
    pub async fn call(
        &self,
        interface: &str,
        method: &str,
        version: &str,
        params: &[(String, String)],
    ) -> Result<Value> {
        let mut url = format!("https://{STEAM_API_HOST}/{interface}/{method}/{version}/");
        if let Some(key) = &self.api_key {
            url.push_str("key=");
            url.push_str(&url_encode(key));
        }
        for (name, value) in params {
            url.push('&');
            url.push_str(name);
            url.push('=');
            url.push_str(&url_encode(value));
        }
        let response = self.http.get(&url).send().await?;
        Ok(response.json::<Value>().await?)
    }

    /// Calls any Steam Web API endpoint and deserializes the JSON body.
    pub async fn call_typed<T: DeserializeOwned>(
        &self,
        interface: &str,
        method: &str,
        version: &str,
        params: &[(String, String)],
    ) -> Result<T> {
        let value = self.call(interface, method, version, params).await?;
        Ok(serde_json::from_value(value)?)
    }

    /// ISteamWebAPIUtil/GetServerTime: current Steam server time (unix seconds).
    pub async fn get_server_time(&self) -> Result<u64> {
        let value = self
            .call("ISteamWebAPIUtil", "GetServerTime", "v2", &[])
            .await?;
        value
            .get("response")
            .and_then(|r| r.get("server_time"))
            .and_then(Value::as_u64)
            .ok_or_else(|| {
                SteamError::Api("missing server_time in GetServerTime response".to_owned())
            })
    }

    /// ISteamApps/GetAppDetails: full app details for the given app id.
    ///
    /// The body is a map keyed by the app id string.
    pub async fn get_app_details(&self, app_id: u32) -> Result<Value> {
        self.call(
            "ISteamApps",
            "GetAppDetails",
            "v0001",
            &[("appid".to_owned(), app_id.to_string())],
        )
        .await
    }
}

impl Deref for SteamApi {
    type Target = SteamClient;

    fn deref(&self) -> &SteamClient {
        &self.client
    }
}

/// Percent-encodes a query string value (RFC 3986 unreserved set stays as-is).
fn url_encode(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(char::from(byte))
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_encodes_reserved_chars() {
        assert_eq!(url_encode("abc-123._~"), "abc-123._~");
        assert_eq!(url_encode("a b&c=d"), "a%20b%26c%3Dd");
        assert_eq!(url_encode("~!@#$^&*()"), "~%21%40%23%24%5E%26%2A%28%29");
        assert_eq!(url_encode("/?:"), "%2F%3F%3A");
    }

    #[test]
    fn client_basics() {
        let api = SteamApi::new();
        assert_eq!(api.api_key(), None);
        let keyed = SteamApi::with_api_key("REDACTED");
        assert_eq!(keyed.api_key(), Some("REDACTED"));
    }
}
