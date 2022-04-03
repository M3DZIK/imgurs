macro_rules! api_url (
    ($path: expr) => (
        format!("{}{}", "https://api.imgur.com/3/", $path)
    );
);

use std::{fmt, fs, io, path::Path};

use anyhow::Error;
pub(crate) use api_url;
use reqwest::Client;

use super::*;

pub struct ImgurClient {
    pub client_id: String,
    pub client: Client,
}

impl fmt::Debug for ImgurClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImgurClient - client_id: {}", self.client_id)
    }
}

impl ImgurClient {
    pub fn new(client_id: String) -> Self {
        let client = Client::new();
        ImgurClient { client_id, client }
    }

    pub async fn upload_image(&self, path: String) -> Result<ImageInfo, Error> {
        let mut image: String = path.clone();

        // check if the specified file exists if not then check if it is a url
        if Path::new(&path).exists() {
            image = fs::read_to_string(&path)
                .map_err(|err| err.to_string())
                .expect("read file");
        } else if !validator::validate_url(&path) {
            let err = io::Error::new(
                io::ErrorKind::Other,
                format!("{path} is not url or file path"),
            );

            return Err(err.into());
        }

        upload_image(self, image).await
    }

    pub async fn delete_image(&self, delete_hash: String) -> Result<(), Error> {
        delete_image(self, delete_hash).await
    }

    pub async fn rate_limit(&self) -> Result<RateLimitInfo, Error> {
        rate_limit(self).await
    }

    pub async fn image_info(&self, id: String) -> Result<ImageInfo, Error> {
        get_image(self, id).await
    }
}
