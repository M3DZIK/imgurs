macro_rules! api_url (
    ($path: expr) => (
        format!("{}{}", "https://api.imgur.com/3/", $path)
    );
);

pub(crate) use api_url;

use std::{fmt, fs, path::Path};

use reqwest::Client;

use crate::{Result, Error, requests::{self, RateLimitInfo}, ImageInfo};

/// Imgur Client
#[derive(Clone)]
pub struct ImgurClient {
    /// Client id for a Imgur API
    pub client_id: String,
    /// HTTP Client
    pub client: Client,
}

impl fmt::Debug for ImgurClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImgurClient - client_id: {}", self.client_id)
    }
}

impl ImgurClient {
    /// Create a new Imgur Client
    /// ```
    /// use imgurs::ImgurClient;
    ///
    /// let client = ImgurClient::new("3e3ce0d7ac14d56");
    /// ```
    pub fn new(client_id: &str) -> Self {
        let client_id = client_id.to_string();
        let client = Client::new();
        ImgurClient { client_id, client }
    }

    /// Upload image to Imgur
    /// ```
    /// use imgurs::ImgurClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = ImgurClient::new("3e3ce0d7ac14d56");
    ///
    ///     client.upload_image("https://i.imgur.com/lFaGr1x.png").await.expect("upload image");
    /// }
    /// ```
    pub async fn upload_image(&self, path: &str) -> Result<ImageInfo> {
        let mut image = path.to_string();

        // check if the specified file exists if not then check if it is a url
        if Path::new(path).exists() {
            let bytes = fs::read(path)?;
            image = base64::encode(bytes)
        }
        // validate url adress
        else if !validator::validate_url(path) {
            Err(Error::InvalidUrlOrFile(path.to_string()))?;
        }

        requests::upload_image(self, image).await
    }

    /// Delete image from Imgur
    /// ```
    /// use imgurs::ImgurClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = ImgurClient::new("3e3ce0d7ac14d56");
    ///
    ///     let image = client.upload_image("https://i.imgur.com/lFaGr1x.png").await.expect("upload image");
    ///     let deletehash = image.data.deletehash.unwrap();
    ///
    ///     client.delete_image(&deletehash).await.expect("delete image");
    /// }
    /// ```
    pub async fn delete_image(&self, delete_hash: &str) -> Result<()> {
        requests::delete_image(self, delete_hash).await
    }

    /// Get Rame Limit of this Imgur Client
    /// ```
    /// use imgurs::ImgurClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = ImgurClient::new("3e3ce0d7ac14d56");
    ///
    ///     client.rate_limit().await.expect("get rate limit");
    /// }
    /// ```
    pub async fn rate_limit(&self) -> Result<RateLimitInfo> {
        requests::rate_limit(self).await
    }

    /// Get image info from a Imgur
    /// ```
    /// use imgurs::ImgurClient;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = ImgurClient::new("3e3ce0d7ac14d56");
    ///
    ///     client.image_info("lFaGr1x").await.expect("delete image");
    /// }
    /// ```
    pub async fn image_info(&self, id: &str) -> Result<ImageInfo> {
        requests::get_image(self, id).await
    }
}
