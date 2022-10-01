macro_rules! api_url (
    ($path: expr) => (
        format!("{}{}", "https://api.imgur.com/3/", $path)
    );
);

use std::fmt;

pub(crate) use api_url;
use reqwest::Client;

/// Imgur Client
#[derive(Clone)]
pub struct ImgurClient {
    /// Imgur API Client ID
    pub client_id: String,
    /// HTTP Client
    pub client: Client,
}

impl fmt::Debug for ImgurClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImgurClient - client_id: {}", self.client_id)
    }
}
