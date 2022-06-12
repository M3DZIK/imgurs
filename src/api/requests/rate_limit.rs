use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{api_url, send_api_request, Error, ImgurClient, Result};

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

pub async fn rate_limit(client: &ImgurClient) -> Result<RateLimitInfo> {
    // get imgur api url
    let uri = api_url!("credits");

    // send request to imgur api
    let res = send_api_request(client, Method::GET, uri, None).await?;

    // get response http code
    let status = res.status();

    // check if an error has occurred
    if status.is_client_error() || status.is_server_error() {
        let body = res.text().await?;

        return Err(Error::ApiError(status.as_u16(), body));
    }

    // return `RateLimitInfo`
    Ok(res.json().await?)
}
