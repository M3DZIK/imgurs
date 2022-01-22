use crate::api::configuration::{api_url, ImgurHandle};

use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageInfo {
    pub data: ImageInfoData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageInfoData {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub datetime: i32,
    #[serde(rename = "type")]
    pub img_type: String,
    pub animated: bool,
    pub width: i32,
    pub height: i32,
    pub size: i32,
    pub views: i32,
    pub bandwidth: i64,
    pub favorite: bool,
    pub deletehash: String,
    pub link: String,
}

pub async fn upload_image(c: ImgurHandle, image: &str) -> Result<ImageInfo, String> {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

    let mut form = HashMap::new();

    form.insert("image", image.to_string());

    let res = c
        .client
        .post(api_url!("image"))
        .header("Authorization", format!("Client-ID {}", c.client_id))
        .header(
            "User-Agent",
            format!("Imgurs/{:?}", VERSION.unwrap_or("unknown")),
        )
        .form(&form)
        .send()
        .await
        .map_err(|err| err.to_string())?;

    let status = res.status();

    if status.is_client_error() || status.is_server_error() {
        let body = res.text().await.map_err(|err| err.to_string())?;
        Err(format!(
            "server returned non-successful status code = {status}. body = {body}"
        ))
    } else {
        let content: ImageInfo = res.json().await.map_err(|err| err.to_string())?;
        Ok(content)
    }
}
