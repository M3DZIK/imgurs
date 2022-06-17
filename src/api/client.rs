macro_rules! api_url (
    ($path: expr) => (
        format!("{}{}", "https://api.imgur.com/3/", $path)
    );
);

pub(crate) use api_url;

use std::fmt;

use reqwest::Client;

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
