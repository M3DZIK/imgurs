use std::collections::HashMap;

use reqwest::{Client, Method};

use crate::{Error, Result};

#[derive(Debug, Clone)]
/// The Null Pointer instance for https://0x0.st
pub struct NullPointer {
    client: Client,
    url: String,
}

impl NullPointer {
    /// Create a new [NullPointer] instance.
    /// ```rs
    /// use imgurs::NullPointer;
    ///
    /// let client = NullPointer::new("https://0x0.st");
    /// ```
    pub fn new(url: &str) -> Self {
        let client = reqwest::Client::new();
        Self {
            client,
            url: url.to_string(),
        }
    }

    /// Upload image to Imgur
    /// ```no_run
    /// use imgurs::NullPointer;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = NullPointer::new("https://0x0.st");
    ///
    ///     client.upload("path/to/file.png").await.expect("failed to upload image");
    /// }
    /// ```
    pub async fn upload(&self, path: &str) -> Result<String> {
        let mut bytes = Vec::new();

        // check if the specified file exists if not then check if it is a url
        if std::path::Path::new(path).exists() {
            bytes = std::fs::read(path)?;
        }
        // validate url adress
        else {
            Err(Error::InvalidUrlOrFile(path.to_string()))?;
        }

        // send request to imgur api
        // get http client
        let client = &self.client;

        // create Request buidler
        let mut req = client.request(Method::POST, &self.url);

        // add `Authorization` and `User-Agent` to Request
        req = req.header(
            "User-Agent",
            format!("Imgur/{:?}", env!("CARGO_PKG_VERSION")),
        );

        let part = reqwest::multipart::Part::bytes(bytes);

        let form = reqwest::multipart::Form::new().part("file", part);

        req = req.multipart(form);

        // build Request
        let req = req.build()?;

        // send Request
        let res = client.execute(req).await?;

        // get response http code
        let status = res.status();

        // check if an error has occurred
        if status.is_client_error() || status.is_server_error() {
            let body = res.text().await?;

            return Err(Error::ApiError(status.as_u16(), body));
        }

        Ok(res.json().await?)
    }
}
