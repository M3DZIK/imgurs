mod cli;
mod config;

use cli::parse::parse;
use imgurs::ImgurClient;

use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().expect("init SimpleLogger");
    better_panic::install();

    let config = config::toml::parse();
    let client = ImgurClient::new((&config.imgur.id).to_string());

    parse(client).await
}
