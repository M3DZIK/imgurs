use super::send_api_request;
use crate::api::configuration::{api_url, ImgurHandle};
use crate::api::ImageInfo;

use log::error;
use reqwest::Method;
use std::process::exit;

pub async fn get_image(c: ImgurHandle, image: &str) -> Result<ImageInfo, String> {
    let uri = api_url!(format!("image/{image}"));
    let res = send_api_request(&c, Method::GET, uri, None)
        .await
        .unwrap_or_else(|e| {
            error!("send api request: {e}");
            exit(1)
        });

    let status = res.status();

    if status.is_client_error() || status.is_server_error() {
        Err(format!(
            "server returned non-successful status code = {status}."
        ))
    } else {
        let content: ImageInfo = res.json().await.map_err(|e| e.to_string())?;
        Ok(content)
    }
}
