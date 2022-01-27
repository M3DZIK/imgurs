use super::Config;

use anyhow::Error;
use dirs::config_dir;
use log::warn;
use std::{
    fs::{create_dir_all, read_to_string, File},
    io::Write as _,
    path::Path,
};
use toml::from_str as toml_from_str;

const CONFIG_DIR: &str = "/imgurs/config.toml";

pub fn parse() -> Config {
    toml().unwrap_or_else(|e| {
        warn!("Parse toml config: {e}! Creating config file...");

        let default_config = include_str!(concat!("../../config.toml"));

        let sys_config_dir = config_dir().expect("find config dir");
        let config_dir = format!("{}{CONFIG_DIR}", sys_config_dir.to_string_lossy());
        let config_path = Path::new(&config_dir);

        let prefix = config_path.parent().unwrap();
        create_dir_all(prefix).expect("create config dir");

        let mut file_ref = File::create(config_path).expect("create failed");

        file_ref
            .write_all(default_config.as_bytes())
            .expect("failed write default config to file");

        toml().expect("parse toml config")
    })
}

fn toml() -> Result<Config, Error> {
    let config_dir = config_dir().unwrap();
    let file_dir = format!("{}{CONFIG_DIR}", config_dir.to_string_lossy());
    let toml_str = read_to_string(file_dir).map_err(Error::new)?;
    let decode = toml_from_str(&toml_str).map_err(Error::new)?;

    Ok(decode)
}
