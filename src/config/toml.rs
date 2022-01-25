use dirs::config_dir;
use toml::from_str;

use super::Config;
use log::{error, warn};

use std::{
    fs::{create_dir_all, read_to_string, File},
    io::Write as _,
    path::Path,
    process::exit,
};

pub fn parse() -> Config {
    toml().unwrap_or_else(|e| {
        warn!("Parse toml config: {e}! Creating config file...");

        let default_config = include_str!(concat!("../../config.toml"));

        let sys_config_dir = config_dir().unwrap();
        let config_dir = format!("{}/imgurs/config.toml", sys_config_dir.to_string_lossy());
        let config_path = Path::new(&config_dir);

        let prefix = config_path.parent().unwrap();
        create_dir_all(prefix).unwrap();

        let mut file_ref = File::create(config_path).expect("create failed");

        file_ref
            .write_all(default_config.as_bytes())
            .expect("failed write default config to file");

        toml().unwrap_or_else(|e| {
            error!("parse toml config: {e}");
            exit(2);
        })
    })
}

fn toml() -> Result<Config, String> {
    let config_dir = config_dir().unwrap();

    let file_dir: String = String::from(config_dir.to_string_lossy()) + "/imgurs/config.toml";

    let toml_str = read_to_string(file_dir).map_err(|err| err.to_string())?;

    let decode = from_str(&toml_str).map_err(|err| err.to_string())?;

    Ok(decode)
}
