use crate::api::configuration::{api_url, ImgurHandle};

use super::send_api_request;

use anyhow::Error as anyhow_err;
use reqwest::Method;
use std::io::{Error, ErrorKind};

pub async fn delete_image(c: ImgurHandle, delete_hash: String) -> Result<String, anyhow_err> {
    let uri = api_url!(format!("image/{delete_hash}"));
    let res = send_api_request(&c, Method::DELETE, uri, None).await?;

    let status = res.status();

    if status.is_client_error() || status.is_server_error() {
        let body = res.text().await.map_err(anyhow_err::new)?;
        let err = Error::new(
            ErrorKind::Other,
            format!("server returned non-successful status code = {status}, body = {body}"),
        );

        Err(anyhow_err::from(err))
    } else {
        Ok("If the delete hash was correct the image was deleted!".to_string())
    }
}
