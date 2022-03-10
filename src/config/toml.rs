use super::Config;

use anyhow::Error;
use colored::Colorize;
use dirs::config_dir;
use log::warn;
use std::{
    fs::{create_dir_all, read_to_string, File},
    io::{Write as _, self},
    path::Path,
};
use toml::from_str as toml_from_str;

const CONFIG_DIR: &str = "/imgurs/config.toml";

pub fn parse() -> Config {
    toml().unwrap_or_else(|err| {
        let mut stdout = std::io::stdout();

        write!(stdout, "{}", "The configuration file could not be opened. Do you want to create/overwrite with DEFAULT values? (Y/n): ".yellow()).unwrap();
        stdout.flush().unwrap();

        let mut value = String::new();
        io::stdin()
            .read_line(&mut value)
            .expect("failed to read line");

        if value.to_lowercase() != "n\n" {
            warn!("Parse toml config: {err}! Creating config file...");

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
        } else {
            panic!("New config creation cancelled!")
        }
    })
}

fn toml() -> Result<Config, Error> {
    let config_dir = config_dir().unwrap();
    let file_dir = format!("{}{CONFIG_DIR}", config_dir.to_string_lossy());
    let toml_str = read_to_string(file_dir).map_err(Error::new)?;
    let decode = toml_from_str(&toml_str).map_err(Error::new)?;

    Ok(decode)
}
