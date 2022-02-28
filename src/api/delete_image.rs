use super::send_api_request;
use crate::api::configuration::{api_url, ImgurClient};

use reqwest::Method;
use std::io::{Error, ErrorKind};

pub async fn delete_image(c: ImgurClient, delete_hash: String) -> Result<String, anyhow::Error> {
    let uri = api_url!(format!("image/{delete_hash}"));
    let res = send_api_request(&c, Method::DELETE, uri, None).await?;

    let status = res.status();

    if status.is_client_error() || status.is_server_error() {
        let body = res.text().await.map_err(anyhow::Error::new)?;
        let err = Error::new(
            ErrorKind::Other,
            format!("server returned non-successful status code = {status}, body = {body}"),
        );

        Err(anyhow::Error::from(err))
    } else {
        Ok("If the delete hash was correct the image was deleted!".to_string())
    }
}
