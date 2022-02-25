use serde_derive::Deserialize;

pub mod toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub imgur: ConfigImgur,
    pub notification: ConfigNotification,
    pub clipboard: ConfigClipboard,
}

#[derive(Debug, Deserialize)]
pub struct ConfigImgur {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfigNotification {
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct ConfigClipboard {
    pub enabled: bool,
}
