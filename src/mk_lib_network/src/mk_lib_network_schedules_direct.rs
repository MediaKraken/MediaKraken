use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha1::{Digest, Sha1};
use std::fmt::{Display, Formatter};

/// Schedules Direct JSON API, version 20141201 (current stable):
/// https://github.com/SchedulesDirect/JSON-Service/wiki/API-20141201
const SCHEDULES_DIRECT_API_BASE: &str = "https://json.schedulesdirect.org/20141201";

/// The API requires a User-Agent header containing the client name and
/// version on every request.
const SCHEDULES_DIRECT_USER_AGENT: &str = "MediaKraken/0.1.0";

#[derive(Debug)]
pub enum SchedulesDirectError {
    Http(reqwest::Error),
    Api { status: StatusCode, message: String },
    AuthenticationTokenMissing,
}

impl Display for SchedulesDirectError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http(err) => write!(f, "http error: {err}"),
            Self::Api { status, message } => {
                write!(f, "schedules direct api error ({status}): {message}")
            }
            Self::AuthenticationTokenMissing => {
                write!(f, "schedules direct token response did not contain a token")
            }
        }
    }
}

impl std::error::Error for SchedulesDirectError {}

impl From<reqwest::Error> for SchedulesDirectError {
    fn from(value: reqwest::Error) -> Self {
        Self::Http(value)
    }
}

pub struct SchedulesDirectClient {
    http_client: reqwest::Client,
    base_url: &'static str,
    token: String,
    token_expires: Option<i64>,
}

impl std::fmt::Debug for SchedulesDirectClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SchedulesDirectClient")
            .field("http_client", &self.http_client)
            .field("base_url", &self.base_url)
            .field("token", &"REDACTED")
            .finish()
    }
}

#[derive(Debug, Deserialize)]
pub struct SchedulesDirectStatus {
    #[serde(rename = "systemStatus")]
    pub system_status: Vec<SchedulesDirectSystemStatus>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Deserialize)]
pub struct SchedulesDirectSystemStatus {
    pub date: String,
    pub status: String,
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Serialize)]
struct SchedulesDirectLoginRequest<'a> {
    username: &'a str,
    password: &'a str,
}

#[derive(Debug, Deserialize)]
struct SchedulesDirectTokenResponse {
    token: Option<String>,
    /// UNIX epoch time at which the token expires.
    #[serde(rename = "tokenExpires")]
    token_expires: Option<i64>,
}

/// One element of the `/schedules` request body. Omitting `date` asks the
/// server for "today" through the last day of schedule data on the server.
#[derive(Debug, Serialize)]
struct SchedulesDirectScheduleRequest<'a> {
    #[serde(rename = "stationID")]
    station_id: &'a str,
}

impl SchedulesDirectClient {
    /// Log in and obtain a token.
    ///
    /// The API expects the SHA-1 hex digest of the password (40 lowercase
    /// characters), not the plaintext password. Tokens are valid for 24
    /// hours; repeated logins with the same credentials return the existing
    /// token, and more than 100 token requests per 24 hours are rejected.
    pub async fn login(username: &str, password: &str) -> Result<Self, SchedulesDirectError> {
        // The gzip feature of the shared client sends `Accept-Encoding: gzip`
        // and decompresses responses; the API requires an Accept-Encoding of
        // gzip (or deflate) on every request.
        let http_client = reqwest::Client::builder()
            .user_agent(SCHEDULES_DIRECT_USER_AGENT)
            .build()?;

        let password_hash: String = Sha1::digest(password.as_bytes())
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        let login_payload = SchedulesDirectLoginRequest {
            username,
            password: &password_hash,
        };

        let response = http_client
            .post(format!("{SCHEDULES_DIRECT_API_BASE}/token"))
            .json(&login_payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let message = response.text().await.unwrap_or_default();
            return Err(SchedulesDirectError::Api { status, message });
        }

        let token_response = response.json::<SchedulesDirectTokenResponse>().await?;
        let token = token_response
            .token
            .ok_or(SchedulesDirectError::AuthenticationTokenMissing)?;

        Ok(Self {
            http_client,
            base_url: SCHEDULES_DIRECT_API_BASE,
            token,
            token_expires: token_response.token_expires,
        })
    }

    /// UNIX epoch time at which the token expires, when provided by the
    /// server in the token response.
    pub fn token_expires(&self) -> Option<i64> {
        self.token_expires
    }

    pub async fn status(&self) -> Result<SchedulesDirectStatus, SchedulesDirectError> {
        self.get_json("status").await
    }

    pub async fn lineups(&self) -> Result<Value, SchedulesDirectError> {
        self.get_json("lineups").await
    }

    pub async fn lineup_channel_map(&self, lineup: &str) -> Result<Value, SchedulesDirectError> {
        self.get_json(&format!("lineups/{lineup}")).await
    }

    /// Download the schedules for the given station IDs.
    ///
    /// The API expects a JSON array of `{"stationID": ...}` objects (with an
    /// optional per-station `date` array of individual `YYYY-MM-DD` dates); a
    /// bare array of station ID strings is rejected. No more than 5000
    /// station IDs per call.
    pub async fn schedules_by_station_ids(
        &self,
        station_ids: &[String],
    ) -> Result<Value, SchedulesDirectError> {
        let payload: Vec<SchedulesDirectScheduleRequest> = station_ids
            .iter()
            .map(|station_id| SchedulesDirectScheduleRequest {
                station_id: station_id.as_str(),
            })
            .collect();
        self.post_json("schedules", &payload).await
    }

    /// Download program metadata for the given program IDs. The API accepts a
    /// bare JSON array of program ID strings; no more than 5000 per call.
    pub async fn program_details(
        &self,
        program_ids: &[String],
    ) -> Result<Value, SchedulesDirectError> {
        self.post_json("programs", program_ids).await
    }

    async fn get_json<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
    ) -> Result<T, SchedulesDirectError> {
        let response = self
            .http_client
            .get(format!("{}/{}", self.base_url, path))
            .header("token", &self.token)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let message = response.text().await.unwrap_or_default();
            return Err(SchedulesDirectError::Api { status, message });
        }

        Ok(response.json::<T>().await?)
    }

    async fn post_json<TReq: Serialize + ?Sized, TResp: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        payload: &TReq,
    ) -> Result<TResp, SchedulesDirectError> {
        let response = self
            .http_client
            .post(format!("{}/{}", self.base_url, path))
            .header("token", &self.token)
            .json(payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let message = response.text().await.unwrap_or_default();
            return Err(SchedulesDirectError::Api { status, message });
        }

        Ok(response.json::<TResp>().await?)
    }
}
