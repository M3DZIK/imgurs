use std::io::stdout;

use clap::{Command, CommandFactory, Parser};
use clap_complete::{generate, Generator, Shell};
use imgurs::ImgurClient;
use simple_logger::SimpleLogger;

use crate::imgur::*;

mod config;
mod imgur;

#[derive(Parser, Debug)]
#[clap(
    name = "imgurs",
    about = "Imgur API CLI",
    long_about = env!("CARGO_PKG_DESCRIPTION"),
    version = env!("CARGO_PKG_VERSION"),
)]
enum Cli {
    #[clap(about = "Print Client Rate Limit", display_order = 1)]
    Credits,

    #[clap(about = "Upload image to Imgur", display_order = 2)]
    Upload { path: String },

    #[clap(about = "Delete image from Imgur", display_order = 3)]
    Delete { delete_hash: String },

    #[clap(about = "Print image info", display_order = 4)]
    Info { id: String },

    #[clap(
        about = "Generate completion file for a shell [bash, elvish, fish, powershell, zsh]",
        display_order = 5
    )]
    Completions { shell: Shell },

    #[clap(about = "Generate man page", display_order = 6)]
    Manpage,
}

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    // parse config file
    let config = config::toml::parse();

    // create imgur client
    let client = ImgurClient::new(&config.imgur.id);

    let args = Cli::parse();

    match args {
        Cli::Credits => credits(client).await,

        Cli::Upload { path } => upload_image(client, path).await,

        Cli::Delete { delete_hash } => delete_image(client, delete_hash.to_string()).await,

        Cli::Info { id } => image_info(client, id.to_string()).await,

        Cli::Completions { shell } => {
            let mut app = Cli::command();

            fn print_completions<G: Generator>(gen: G, app: &mut Command) {
                generate(gen, app, app.get_name().to_string(), &mut stdout())
            }

            print_completions(shell, &mut app)
        },

        Cli::Manpage => {
            let clap_app = Cli::command();
            let man = clap_mangen::Man::new(clap_app);

            man.render(&mut stdout())
                .expect("Failed to generate man page");
        },
    }
}
