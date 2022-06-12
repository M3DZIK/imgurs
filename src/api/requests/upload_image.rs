use std::collections::HashMap;

use reqwest::Method;

use crate::{api_url, send_api_request, Error, ImageInfo, ImgurClient, Result};

pub async fn upload_image(client: &ImgurClient, image: String) -> Result<ImageInfo> {
    // create http form (hashmap)
    let mut form = HashMap::new();
    // insert image to form
    form.insert("image", image);

    // get imgur api url
    let uri = api_url!("image");

    // send request to imgur api
    let res = send_api_request(client, Method::POST, uri, Some(form)).await?;

    // get response http code
    let status = res.status();

    // check if an error has occurred
    if status.is_client_error() || status.is_server_error() {
        let body = res.text().await?;

        // if body is too long do not return it (imgur sometimes returns whole Request)
        if body.chars().count() > 50 {
            Err(Error::ApiErrorBodyTooLong(status.as_u16()))?;
        }

        return Err(Error::ApiError(status.as_u16(), body));
    }

    // return `ImageInfo`
    Ok(res.json().await?)
}
