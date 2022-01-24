use serde_derive::Deserialize;

pub mod toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub imgur: ConfigImgur,
    pub notification: ConfigNotification,
}

#[derive(Debug, Deserialize)]
pub struct ConfigImgur {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfigNotification {
    pub enabled: bool,
}
