use std::{collections::HashMap, io};

use anyhow::Error;
use reqwest::Method;

use super::{client::api_url, send_api_request, ImageInfo, ImgurClient};

pub async fn upload_image(c: &ImgurClient, image: String) -> Result<ImageInfo, Error> {
    // create http form (hashmap)
    let mut form = HashMap::new();
    // insert image to form
    form.insert("image", image);

    // get imgur api url
    let uri = api_url!("image");

    // send request to imgur api
    let res = send_api_request(&c, Method::POST, uri, Some(form)).await?;

    // get response http code
    let status = res.status();

    // check if an error has occurred
    if status.is_client_error() || status.is_server_error() {
        let mut body = res.text().await?;

        if body.chars().count() > 200 {
            body = "server returned too long".to_string()
        }

        let err = io::Error::new(
            io::ErrorKind::Other,
            format!("server returned non-successful status code = {status}, body = {body}"),
        );

        Err(err)?
    } else {
        let content: ImageInfo = res.json().await?;
        Ok(content)
    }
}
