use super::ImgurHandle;

use anyhow::Error;
use reqwest::Method;
use std::collections::HashMap;

pub async fn send_api_request(
    config: &ImgurHandle,
    method: Method,
    uri: String,
    form: Option<HashMap<&str, String>>,
) -> Result<reqwest::Response, Error> {
    let client = &config.client;

    let mut req = client.request(method, uri.as_str());

    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

    req = req
        .header("Authorization", format!("Client-ID {}", config.client_id))
        .header(
            "User-Agent",
            format!("Imgur/{:?}", VERSION.unwrap_or("unknown")),
        );

    if form != None {
        req = req.form(&form.unwrap())
    }

    let req = req.build()?;

    client.execute(req).await.map_err(Error::from)
}
