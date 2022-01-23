mod cli;
mod config;

use cli::parse::parse;

use log::error;
use simple_logger::SimpleLogger;
use std::process::exit;

use imgurs::api::ImgurHandle;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    let config = config::toml::parse().unwrap_or_else(|error| {
        error!("Parse toml config: {}", error);
        exit(1);
    });

    let client = ImgurHandle::new((&config.imgur.id).to_string());

    parse(client).await;
}
