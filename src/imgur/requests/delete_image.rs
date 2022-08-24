use reqwest::Method;

use crate::{api_url, send_api_request, Error, ImgurClient, Result};

pub async fn delete_image(client: &ImgurClient, delete_hash: &str) -> Result<()> {
    // get imgur api url
    let uri = api_url!(format!("image/{delete_hash}"));

    // send request to imgur api
    let res = send_api_request(client, Method::DELETE, uri, None).await?;

    // get response http code
    let status = res.status();

    // check if an error has occurred
    if status.is_client_error() || status.is_server_error() {
        let body = res.text().await?;

        Err(Error::ApiError(status.as_u16(), body))?;
    }

    Ok(())
}
