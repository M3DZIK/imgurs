use std::io;

use reqwest::Method;

use super::{client::api_url, send_api_request, ImgurClient};

pub async fn delete_image(client: &ImgurClient, delete_hash: &str) -> anyhow::Result<()> {
    // get imgur api url
    let uri = api_url!(format!("image/{delete_hash}"));

    // send request to imgur api
    let res = send_api_request(client, Method::DELETE, uri, None).await?;

    // get response http code
    let status = res.status();

    // check if an error has occurred
    if status.is_client_error() || status.is_server_error() {
        let mut body = res.text().await?;

        if body.chars().count() > 30 {
            body = "body is too length".to_string()
        }

        let err = io::Error::new(
            io::ErrorKind::Other,
            format!("server returned non-successful status code = {status}, body = {body}"),
        );

        return Err(err.into());
    }

    Ok(())
}
