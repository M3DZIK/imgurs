use reqwest::Client;
use std::fmt;

macro_rules! api_url (
    ($path: expr) => (
        format!("{}{}", "https://api.imgur.com/3/", $path)
    );
);

pub(crate) use api_url;

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
}
