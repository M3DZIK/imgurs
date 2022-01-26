use crate::api::configuration::{api_url, ImgurHandle};
use crate::api::ImageInfo;

use super::send_api_request;

use anyhow::Error as anyhow_err;
use reqwest::Method;
use std::io::{Error, ErrorKind};

pub async fn get_image(c: ImgurHandle, image: &str) -> Result<ImageInfo, anyhow_err> {
    let uri = api_url!(format!("image/{image}"));
    let res = send_api_request(&c, Method::GET, uri, None).await?;

    let status = res.status();

    if status.is_client_error() || status.is_server_error() {
        let err = Error::new(
            ErrorKind::Other,
            format!("server returned non-successful status code = {status}"),
        );

        Err(anyhow_err::from(err))
    } else {
        let content: ImageInfo = res.json().await.map_err(anyhow_err::new)?;
        Ok(content)
    }
}
