use clap::{AppSettings, Parser, Subcommand};

use imgurs::api::configuration::ImgurHandle;

use crate::cli::*;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[clap(
    name = "imgur",
    about = "Imgur API CLI", long_about = None,
    version = VERSION.unwrap_or("unknown")
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about = "Get API Rate Limit")]
    Credits,

    #[clap(
        setting(AppSettings::ArgRequiredElseHelp),
        about = "Upload image to imgur"
    )]
    Upload { path: String },

    #[clap(
        setting(AppSettings::ArgRequiredElseHelp),
        about = "Upload image to imgur"
    )]
    Delete { delete_hash: String },
}

pub async fn parse(client: ImgurHandle) {
    let args = Cli::parse();

    match &args.command {
        Commands::Credits => {
            credits::credits(client).await;
        }

        Commands::Upload { path } => {
            upload_image::upload_image(client, path).await;
        }

        Commands::Delete { delete_hash } => {
            delete_image::delete_image(client, delete_hash.to_string()).await;
        }
    }
}
