use reqwest::Method;

use crate::{api_url, send_api_request, Error, ImageInfo, ImgurClient, Result};

pub async fn get_image(client: &ImgurClient, image: &str) -> Result<ImageInfo> {
    // get imgur api url
    let uri = api_url!(format!("image/{image}"));

    // send request to imgur api
    let res = send_api_request(client, Method::GET, uri, None).await?;

    // get response http code
    let status = res.status();

    // check if an error has occurred
    if status.is_client_error() || status.is_server_error() {
        let body = res.text().await?;

        return Err(Error::ApiError(status.as_u16(), body));
    }

    // return `ImageInfo`
    Ok(res.json().await?)
}
