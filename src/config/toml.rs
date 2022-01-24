use dirs::config_dir;
use serde_derive::Deserialize;
use std::fs::read_to_string;
use toml::from_str;

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
    pub enable: bool,
}

pub fn parse() -> Result<Config, String> {
    let config_dir = config_dir().unwrap();
    let file_dir: String = String::from(config_dir.to_string_lossy()) + "/imgurs/config.toml";

    let toml_str = read_to_string(file_dir).map_err(|err| err.to_string())?;

    let decode = from_str(&toml_str).map_err(|err| err.to_string())?;

    Ok(decode)
}
