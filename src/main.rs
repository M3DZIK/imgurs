use imgurs::ImgurClient;
use simple_logger::SimpleLogger;

mod cli;
mod config;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().expect("init SimpleLogger");
    better_panic::install();

    // parse config file
    let config = config::toml::parse();

    // create imgur client
    let client = ImgurClient::new(config.imgur.id);

    cli::parse(client).await
}
