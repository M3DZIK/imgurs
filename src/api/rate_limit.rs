use super::send_api_request;
use crate::api::configuration::{api_url, ImgurClient};

use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};

#[derive(Debug, Serialize, Deserialize)]
pub struct RateLimitInfo {
    pub data: RateLimitData,
    pub success: bool,
    pub status: i32,
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

pub async fn rate_limit(c: ImgurClient) -> Result<RateLimitInfo, anyhow::Error> {
    let uri = api_url!("credits");
    let res = send_api_request(&c, Method::GET, uri, None).await?;

    let status = res.status();

    if status.is_client_error() || status.is_server_error() {
        let body = res.text().await.map_err(anyhow::Error::new)?;
        let err = Error::new(
            ErrorKind::Other,
            format!("server returned non-successful status code = {status}, body = {body}"),
        );

        Err(anyhow::Error::from(err))
    } else {
        let content: RateLimitInfo = res.json().await.map_err(anyhow::Error::new)?;
        Ok(content)
    }
}
