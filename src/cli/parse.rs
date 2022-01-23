use clap::{AppSettings, Parser, Subcommand};

use imgurs::api::configuration::ImgurHandle;

use crate::cli::{credits::*, delete_image::*, info_image::*, upload_image::*};

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const NAME: Option<&str> = option_env!("CARGO_PKG_NAME");

#[derive(Parser, Debug)]
#[clap(
    name = NAME.unwrap_or("unknown"),
    about = "Imgur API CLI", long_about = None,
    version = VERSION.unwrap_or("unknown")
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(about = "Get API ratelimit")]
    Credits,

    #[clap(
        setting(AppSettings::ArgRequiredElseHelp),
        about = "Upload image to imgur"
    )]
    Upload { path: String },

    #[clap(
        setting(AppSettings::ArgRequiredElseHelp),
        about = "Delete image from imgur"
    )]
    Delete { delete_hash: String },

    #[clap(setting(AppSettings::ArgRequiredElseHelp), about = "Print image info")]
    Info { id: String },
}

pub async fn parse(client: ImgurHandle) {
    let args = Cli::parse();

    match &args.command {
        Commands::Credits => {
            credits(client).await;
        }

        Commands::Upload { path } => {
            upload_image(client, path).await;
        }

        Commands::Delete { delete_hash } => {
            delete_image(client, delete_hash.to_string()).await;
        }

        Commands::Info { id } => {
            image_info(client, id).await;
        }
    }
}
