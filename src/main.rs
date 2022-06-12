use imgurs::ImgurClient;
use simple_logger::SimpleLogger;

mod cli;
mod config;

fn main() {
    // init logger
    SimpleLogger::new().init().expect("init SimpleLogger");
    // init better_panic
    better_panic::install();

    // parse config file
    let config = config::toml::parse();

    // create imgur client
    let client = ImgurClient::new(&config.imgur.id);

    // parse cli
    cli::parse(client)
}
