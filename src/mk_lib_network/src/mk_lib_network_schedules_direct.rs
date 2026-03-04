use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};

const SCHEDULES_DIRECT_API_BASE: &str = "https://json.schedulesdirect.org/20141201";

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
}

impl SchedulesDirectClient {
    pub async fn login(username: &str, password: &str) -> Result<Self, SchedulesDirectError> {
        let http_client = reqwest::Client::builder()
            .user_agent("MediaKraken")
            .build()?;

        let login_payload = SchedulesDirectLoginRequest { username, password };

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
        })
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

    pub async fn schedules_by_station_ids(
        &self,
        station_ids: &[String],
    ) -> Result<Value, SchedulesDirectError> {
        self.post_json("schedules", station_ids).await
    }

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

    // ✅ FIX 2 APPLIED HERE
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