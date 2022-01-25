use super::send_api_request;
use crate::api::configuration::{api_url, ImgurHandle};

use log::error;
use reqwest::Method;
use std::process::exit;

pub async fn delete_image(c: ImgurHandle, delete_hash: String) -> Result<String, String> {
    let uri = api_url!(format!("image/{delete_hash}"));
    let res = send_api_request(&c, Method::DELETE, uri, None)
        .await
        .unwrap_or_else(|e| {
            error!("send api request: {e}");
            exit(1)
        });

    let status = res.status();

    if status.is_client_error() || status.is_server_error() {
        let body = res.text().await.map_err(|err| err.to_string())?;
        Err(format!(
            "server returned non-successful status code = {status}. body = {body}"
        ))
    } else {
        Ok("If the delete hash was correct the image was deleted!".to_string())
    }
}
