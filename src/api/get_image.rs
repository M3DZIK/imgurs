use crate::api::configuration::{api_url, ImgurClient};
use crate::api::ImageInfo;
use super::send_api_request;

use reqwest::Method;
use std::io::{Error, ErrorKind};

pub async fn get_image(c: ImgurClient, image: &str) -> Result<ImageInfo, anyhow::Error> {
    let uri = api_url!(format!("image/{image}"));
    let res = send_api_request(&c, Method::GET, uri, None).await?;

    let status = res.status();

    if status.is_client_error() || status.is_server_error() {
        let err = Error::new(
            ErrorKind::Other,
            format!("server returned non-successful status code = {status}"),
        );

        Err(anyhow::Error::from(err))
    } else {
        let content: ImageInfo = res.json().await.map_err(anyhow::Error::new)?;
        Ok(content)
    }
}
