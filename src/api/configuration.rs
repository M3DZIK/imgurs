use reqwest::Client;
use std::fmt;

macro_rules! api_url (
    ($path: expr) => (
        format!("{}{}", "https://api.imgur.com/3/", $path)
    );
);

pub(crate) use api_url;

pub struct ImgurHandle {
    pub client_id: String,
    pub client: Client,
}

impl fmt::Debug for ImgurHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ImgurClient - client_id: {}", self.client_id)
    }
}

impl ImgurHandle {
    pub fn new(client_id: String) -> Self {
        let client = Client::new();
        ImgurHandle { client_id, client }
    }
}
