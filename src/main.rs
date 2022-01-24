mod cli;
mod config;

use cli::parse::parse;

use dirs::config_dir;
use log::warn;
use simple_logger::SimpleLogger;
use std::{path::Path, fs::create_dir_all};

use std::fs::File;
use std::io::Write as _;

use imgurs::api::ImgurHandle;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    let config = config::toml::parse().unwrap_or_else(|error| {
        warn!("Parse toml config: {}! Creating config file...", error);

        let default_config = include_str!(concat!("../config.toml"));

        let sys_config_dir = config_dir().unwrap();
        let config_dir = format!("{}/imgurs/config.toml", sys_config_dir.to_string_lossy());
        let config_path = Path::new(&config_dir);

        let prefix = config_path.parent().unwrap();
        create_dir_all(prefix).unwrap();

        let mut file_ref = File::create(config_path).expect("create failed");

        file_ref.write_all(default_config.as_bytes()).expect("failed write default config to file");

        config::toml::parse().unwrap()
    });

    let client = ImgurHandle::new((&config.imgur.id).to_string());

    parse(client).await;
}
