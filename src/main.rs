mod cli;
mod config;

use cli::parse::parse;
use imgurs::api::ImgurHandle;

use log::error;
use simple_logger::SimpleLogger;
use std::process::exit;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap_or_else(|e| {
        error!("init simple logger: {e}");
        exit(2)
    });

    let config = config::toml::parse();

    let client = ImgurHandle::new((&config.imgur.id).to_string());

    parse(client).await
}
