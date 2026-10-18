use std::ops::Deref;

use serde::de::DeserializeOwned;
use serde_json::Value;
use xbox::auth::{LegacyPasswordProvider, XblAuthProvider};
use xbox::util::wrap_xuid;
use xbox::{RelyingParty, XboxClient};

use crate::mk_lib_xbox_error::{Result, XboxApiError};

const SOCIAL_PROFILE_BASE_URL: &str = "https://social.xboxlive.com";
const XBL_CONTRACT_VERSION: &str = "1";

/// Entry point for all Xbox Live calls.
///
/// Derefs to [`XboxClient`], so the `xbox` crate's typed calls (`xsts_ticket`,
/// `gamertag_to_xuid`) are available directly, e.g.
/// `api.gamertag_to_xuid("Some Gamertag").await`.
///
/// Any other Xbox Live service endpoint (profile, friends, achievements, …)
/// can be called with [`XboxApi::call`] / [`XboxApi::call_post`]; the
/// `XBL3.0 x=<uhs>;<token>` authorization header is built automatically from
/// a cached XSTS ticket for the requested relying party.
pub struct XboxApi<P: XblAuthProvider> {
    client: XboxClient<P>,
    http: reqwest::Client,
}

impl XboxApi<LegacyPasswordProvider> {
    /// Creates a client using Microsoft account username/password login
    /// (unofficial legacy flow, `xbox` crate `legacy-password-login` feature).
    pub fn with_password(username: &str, password: &str) -> Self {
        Self::with_provider(LegacyPasswordProvider::new(username, password))
    }
}

impl<P: XblAuthProvider> XboxApi<P> {
    /// Creates a client with a custom [`XblAuthProvider`].
    pub fn with_provider(provider: P) -> Self {
        Self {
            client: XboxClient::new(provider),
            http: reqwest::Client::new(),
        }
    }

    /// Creates a client with a caller-supplied `reqwest::Client` (proxy,
    /// timeouts, …) for the generic call path.
    pub fn with_http_client(provider: P, http: reqwest::Client) -> Self {
        Self {
            client: XboxClient::new(provider),
            http,
        }
    }

    /// GETs any Xbox Live service endpoint and returns the JSON body.
    ///
    /// `url` is the full endpoint url, e.g.
    /// `"https://social.xboxlive.com/users/me/social"`. Query parameters are
    /// url-encoded and appended. The XSTS ticket for `relying_party` is
    /// fetched (and cached) and sent as the `Authorization` header.
    pub async fn call(
        &self,
        relying_party: RelyingParty,
        url: &str,
        params: &[(String, String)],
    ) -> Result<Value> {
        let url = build_url(url, params);
        self.get(relying_party, &url).await
    }

    /// GETs any Xbox Live service endpoint and deserializes the JSON body.
    pub async fn call_typed<T: DeserializeOwned>(
        &self,
        relying_party: RelyingParty,
        url: &str,
        params: &[(String, String)],
    ) -> Result<T> {
        let value = self.call(relying_party, url, params).await?;
        Ok(serde_json::from_value(value)?)
    }

    /// POSTs a JSON body to any Xbox Live service endpoint and returns the
    /// JSON body. See [`XboxApi::call`] for the authorization behavior.
    pub async fn call_post(
        &self,
        relying_party: RelyingParty,
        url: &str,
        body: &Value,
    ) -> Result<Value> {
        let header = self.authorization(relying_party).await?;
        let response = self
            .http
            .post(url)
            .header("Authorization", &header)
            .header("x-xbl-contract-version", XBL_CONTRACT_VERSION)
            .header("Accept", "application/json")
            .json(body)
            .send()
            .await?;
        json_body(response).await
    }

    /// POSTs a JSON body to any Xbox Live service endpoint and deserializes
    /// the JSON body.
    pub async fn call_post_typed<T: DeserializeOwned>(
        &self,
        relying_party: RelyingParty,
        url: &str,
        body: &Value,
    ) -> Result<T> {
        let value = self.call_post(relying_party, url, body).await?;
        Ok(serde_json::from_value(value)?)
    }

    /// The player's social profile (profile, friends count, …) from
    /// `social.xboxlive.com/users/{player}/social`.
    ///
    /// `player` is a gamertag or a raw/wrapped XUID; all-digit inputs are
    /// treated as XUIDs and wrapped as `xuid(123)` automatically.
    pub async fn profile(&self, player: &str) -> Result<Value> {
        let player_id = if !player.is_empty() && player.bytes().all(|b| b.is_ascii_digit()) {
            wrap_xuid(player)
        } else {
            player.to_owned()
        };
        let player = url_encode(&player_id);
        let url = format!("{SOCIAL_PROFILE_BASE_URL}/users/{player}/social");
        self.call(RelyingParty::XBOX, &url, &[]).await
    }

    async fn get(&self, relying_party: RelyingParty, url: &str) -> Result<Value> {
        let header = self.authorization(relying_party).await?;
        let response = self
            .http
            .get(url)
            .header("Authorization", &header)
            .header("x-xbl-contract-version", XBL_CONTRACT_VERSION)
            .header("Accept", "application/json")
            .send()
            .await?;
        json_body(response).await
    }

    async fn authorization(&self, relying_party: RelyingParty) -> Result<String> {
        let ticket = self.client.xsts_ticket(relying_party).await?;
        ticket.authorization_header().ok_or_else(|| {
            XboxApiError::Api(
                "XSTS ticket is missing the user hash (uhs) in its display claims".to_owned(),
            )
        })
    }
}

impl<P: XblAuthProvider> Deref for XboxApi<P> {
    type Target = XboxClient<P>;

    fn deref(&self) -> &XboxClient<P> {
        &self.client
    }
}

async fn json_body(response: reqwest::Response) -> Result<Value> {
    let status = response.status();
    let value: Value = response.json().await?;
    if !status.is_success() {
        return Err(XboxApiError::Api(format!(
            "xbox live returned status {status}: {value}"
        )));
    }
    Ok(value)
}

/// Appends url-encoded query parameters to `base`, handling the `?`/`&`
/// separator.
fn build_url(base: &str, params: &[(String, String)]) -> String {
    if params.is_empty() {
        return base.to_owned();
    }
    let separator = if base.contains('?') { '&' } else { '?' };
    let query = params
        .iter()
        .map(|(key, value)| format!("{}={}", url_encode(key), url_encode(value)))
        .collect::<Vec<String>>()
        .join("&");
    format!("{base}{separator}{query}")
}

/// Percent-encodes a value (RFC 3986 unreserved set stays as-is).
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
    use xbox::models::Xuid;

    #[test]
    fn url_encodes_reserved_chars() {
        assert_eq!(url_encode("abc-123._~"), "abc-123._~");
        assert_eq!(url_encode("a b&c=d"), "a%20b%26c%3Dd");
        assert_eq!(url_encode("~!@#$^&*()"), "~%21%40%23%24%5E%26%2A%28%29");
        assert_eq!(url_encode("/?:"), "%2F%3F%3A");
    }

    #[test]
    fn build_url_appends_params() {
        assert_eq!(build_url("https://x.example/a", &[]), "https://x.example/a");
        assert_eq!(
            build_url("https://x.example/a", &[("q".to_owned(), "A B".to_owned())]),
            "https://x.example/a?q=A%20B"
        );
        assert_eq!(
            build_url(
                "https://x.example/a?keep=1",
                &[("q".to_owned(), "x".to_owned())]
            ),
            "https://x.example/a?keep=1&q=x"
        );
    }

    #[test]
    fn xuid_helper_reexports() {
        let xuid = Xuid::from("2535405290989773");
        assert_eq!(xuid.as_str(), "2535405290989773");
        assert_eq!(wrap_xuid("2535405290989773"), "xuid(2535405290989773)");
        assert_eq!(wrap_xuid("Some Gamertag"), "xuid(Some Gamertag)");
    }
}
