use crate::api::{
    configuration::{api_url, ImgurClient},
    ImageInfo,
};

use super::send_api_request;

use anyhow::Error as anyhow_err;
use reqwest::Method;
use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
};

pub async fn upload_image(c: ImgurClient, image: &str) -> Result<ImageInfo, anyhow_err> {
    let mut form = HashMap::new();

    form.insert("image", image.to_string());

    let uri = api_url!("image");
    let res = send_api_request(&c, Method::POST, uri, Some(form)).await?;

    let status = res.status();

    if status.is_client_error() || status.is_server_error() {
        let body = res.text().await.map_err(anyhow_err::new)?;
        let err = Error::new(
            ErrorKind::Other,
            format!("server returned non-successful status code = {status}, body = {body}"),
        );

        Err(anyhow_err::from(err))
    } else {
        let content: ImageInfo = res.json().await.map_err(anyhow_err::new)?;
        Ok(content)
    }
}
