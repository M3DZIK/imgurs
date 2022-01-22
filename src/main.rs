mod cli;
mod config;

use cli::parse::parse;

use simple_logger::SimpleLogger;

use imgurs::api::ImgurHandle;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    let config = config::toml::parse().unwrap();

    let client = ImgurHandle::new((&config.imgur.id).to_string());

    parse(client).await;
}
