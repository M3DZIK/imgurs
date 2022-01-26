use crate::api::{
    configuration::{api_url, ImgurHandle},
    ImageInfo,
};

use super::send_api_request;

use log::error;
use reqwest::Method;
use std::{collections::HashMap, process::exit};

pub async fn upload_image(c: ImgurHandle, image: &str) -> Result<ImageInfo, String> {
    let mut form = HashMap::new();

    form.insert("image", image.to_string());

    let uri = api_url!("image");
    let res = send_api_request(&c, Method::POST, uri, Some(form))
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
        let content: ImageInfo = res.json().await.map_err(|e| e.to_string())?;

        Ok(content)
    }
}
