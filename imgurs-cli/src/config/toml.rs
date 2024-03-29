use std::{
    fs::{create_dir_all, read_to_string, File},
    io::{self, Write as _},
    path::Path,
};

use colored::Colorize;
use dirs::config_dir;
use log::warn;
use toml::from_str as toml_from_str;

use super::Config;

/// Configuration file path (in system config directory).
const CONFIG_DIR: &str = "/imgurs/config.toml";

/// Parse configuration file
pub fn parse() -> Config {
    // parse config or use default
    toml().unwrap_or_else(|err| {
        let mut stdout = std::io::stdout();

        write!(stdout, "{}", "The configuration file could not be opened. Do you want to create/overwrite with DEFAULT values? (Y/n): ".yellow()).unwrap();
        stdout.flush().unwrap();

        let mut value = String::new();
        io::stdin()
            .read_line(&mut value)
            .expect("failed to read line");

        if value.to_lowercase() != "n\n" {
            warn!("Parse toml config error: {err}! Creating config file...");

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
            panic!("Configuration file creation cancelled!")
        }
    })
}

fn toml() -> anyhow::Result<Config> {
    let config_dir = config_dir().unwrap();
    let file_dir = format!("{}{CONFIG_DIR}", config_dir.to_string_lossy());
    let toml_str = read_to_string(file_dir)?;
    let decode = toml_from_str(&toml_str)?;

    Ok(decode)
}
