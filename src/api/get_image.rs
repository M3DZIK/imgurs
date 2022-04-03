use std::io;

use anyhow::Error;
use reqwest::Method;

use super::{client::api_url, send_api_request, ImageInfo, ImgurClient};

pub async fn get_image(client: &ImgurClient, image: String) -> Result<ImageInfo, Error> {
    // get imgur api url
    let uri = api_url!(format!("image/{image}"));

    // send request to imgur api
    let res = send_api_request(client, Method::GET, uri, None).await?;

    // get response http code
    let status = res.status();

    // check if an error has occurred
    if status.is_client_error() || status.is_server_error() {
        let err = io::Error::new(
            io::ErrorKind::Other,
            format!("server returned non-successful status code = {status}"),
        );

        Err(err)?
    } else {
        let content: ImageInfo = res.json().await?;
        Ok(content)
    }
}
