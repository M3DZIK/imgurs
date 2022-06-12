use std::collections::HashMap;

use reqwest::{Method, Response};

use crate::{ImgurClient, Result};

/// Send request to a Imgur API
pub async fn send_api_request(
    config: &ImgurClient,
    method: Method,
    uri: String,
    form: Option<HashMap<&str, String>>,
) -> Result<Response> {
    // get http client
    let client = &config.client;

    // create Request buidler
    let mut req = client.request(method, uri.as_str());

    // get imgurs version
    let version: &str = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");

    // add `Authorization` and `User-Agent` to Request
    req = req
        .header("Authorization", format!("Client-ID {}", config.client_id))
        .header("User-Agent", format!("Imgur/{:?}", version));

    // if exists add HashMap to Request
    if form != None {
        req = req.form(&form.unwrap())
    }

    // build Request
    let req = req.build()?;

    // send Request
    Ok(client.execute(req).await?)
}
