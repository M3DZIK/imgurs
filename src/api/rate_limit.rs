use crate::api::configuration::{api_url, ImgurHandle};

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RateLimitInfo {
    pub data: RateLimitData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RateLimitData {
    #[serde(rename = "UserLimit")]
    pub user_limit: i32,
    #[serde(rename = "UserRemaining")]
    pub user_remaining: i32,
    #[serde(rename = "UserReset")]
    pub user_reset: i32,
    #[serde(rename = "ClientLimit")]
    pub client_limit: i32,
    #[serde(rename = "ClientRemaining")]
    pub client_remaining: i32,
}

pub async fn rate_limit(c: ImgurHandle) -> Result<RateLimitInfo, String> {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

    let res = c
        .client
        .get(api_url!("credits"))
        .header("Authorization", format!("Client-ID {}", c.client_id))
        .header(
            "User-Agent",
            format!("Imgur/{:?}", VERSION.unwrap_or("unknown")),
        )
        .send()
        .await
        .map_err(|err| err.to_string())?;

    let status = res.status();

    if status.is_client_error() || status.is_server_error() {
        let body = res.text().await.map_err(|err| err.to_string())?;
        Err(format!("server returned non-successful status code = {status}. body = {body}"))
    } else {
        let content: RateLimitInfo = res.json().await.map_err(|err| err.to_string())?;
        Ok(content)
    }
}
