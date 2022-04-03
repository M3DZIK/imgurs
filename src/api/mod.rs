mod delete_image;
mod get_image;
mod image_type;
mod rate_limit;
mod upload_image;

pub mod client;

pub use client::ImgurClient;
pub use delete_image::*;
pub use get_image::*;
pub use image_type::*;
pub use rate_limit::*;
pub use upload_image::*;

use std::collections::HashMap;

use reqwest::{Response, Method};
use anyhow::Error;

// send request to imgur api
pub async fn send_api_request(
    config: &ImgurClient,
    method: Method,
    uri: String,
    form: Option<HashMap<&str, String>>,
) -> Result<Response, Error> {
    // get request client
    let client = &config.client;

    // create request buidler
    let mut req = client.request(method, uri.as_str());

    // get program version
    let version: Option<&str> = option_env!("CARGO_PKG_VERSION");
    let version = version.unwrap_or("unknown");

    // add `Authorization` and `User-Agent` to request
    req = req
        .header("Authorization", format!("Client-ID {}", config.client_id))
        .header(
            "User-Agent",
            format!("Imgur/{:?}", version),
        );

    // if exists add hashmap to request
    if form != None {
        req = req.form(&form.unwrap())
    }

    // build request
    let req = req.build()?;

    // send request
    Ok(client.execute(req).await?)
}
