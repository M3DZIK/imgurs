use super::send_api_request;
use crate::api::configuration::{api_url, ImgurHandle};

use log::error;
use reqwest::Method;
use serde_derive::{Deserialize, Serialize};
use std::process::exit;

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

pub async fn rate_limit(c: ImgurHandle) -> Result<RateLimitInfo, String> {
    let uri = api_url!("credits");
    let res = send_api_request(&c, Method::GET, uri, None)
        .await
        .unwrap_or_else(|e| {
            error!("send api request: {e}");
            exit(1)
        });

    let status = res.status();

    if status.is_client_error() || status.is_server_error() {
        let body = res.text().await.map_err(|e| e.to_string())?;
        Err(format!(
            "server returned non-successful status code = {status}. body = {body}"
        ))
    } else {
        let content: RateLimitInfo = res.json().await.map_err(|e| e.to_string())?;
        Ok(content)
    }
}
