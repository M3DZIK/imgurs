use serde::Serialize;
use serde_derive::Deserialize;

pub mod toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub imgur: ConfigImgur,
    pub notification: ConfigNotification,
    pub clipboard: ConfigClipboard,
    pub discord_webhook: ConfigDiscordWebhook,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigImgur {
    pub id: String,
    pub image_cdn: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigNotification {
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigClipboard {
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigDiscordWebhook {
    pub enabled: bool,
    pub uri: String,
}
