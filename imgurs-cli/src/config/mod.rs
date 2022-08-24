use serde::{Deserialize, Serialize};

pub mod toml;

/// Configuration schema
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Imgur API configuration options
    pub imgur: ConfigImgur,
    /// Notification options
    pub notification: ConfigNotification,
    /// Clipboard options
    pub clipboard: ConfigClipboard,
}

/// Imgur API configuration options
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigImgur {
    /// Imgur Client ID
    pub id: String,
    /// Imgur Domain (e.g. if you have a imgur proxy)
    pub image_cdn: String,
}

/// Notification options
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigNotification {
    /// Send notification
    pub enabled: bool,
}

/// Clipboard options
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigClipboard {
    /// Copy image url to clipboard
    pub enabled: bool,
}
