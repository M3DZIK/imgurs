use imgurs::api::ImgurClient;
use clap::{Command, IntoApp, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use std::io::{stdout, self};

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
    #[clap(about = "Print API Rate Limit", display_order = 1)]
    Credits,

    #[clap(about = "Upload image to Imgur", display_order = 2)]
    Upload { path: String },

    #[clap(about = "Delete image from Imgur", display_order = 3)]
    Delete { delete_hash: String },

    #[clap(about = "Print image info", display_order = 4)]
    Info { id: String },

    #[clap(about = "Generate completion file for a shell [bash, elvish, fish, powershell, zsh]", display_order = 5)]
    Completions { shell: String },

    #[clap(about = "Generate man page", display_order = 6)]
    Manpage,
}

fn print_completions<G: Generator>(gen: G, app: &mut Command) {
    generate(gen, app, app.get_name().to_string(), &mut stdout())
}

pub async fn parse(client: ImgurClient) {
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

        Commands::Completions { shell } => {
            let mut app = Cli::command();

            match shell.as_str() {
                "bash" => print_completions(Shell::Bash, &mut app),
                "elvish" => print_completions(Shell::Elvish, &mut app),
                "fish" => print_completions(Shell::Fish, &mut app),
                "powershell" => print_completions(Shell::PowerShell, &mut app),
                "zsh" => print_completions(Shell::Zsh, &mut app),

                _ => panic!("Completions to shell `{shell}`, not found!"),
            }
        }

        Commands::Manpage => {
            let clap_app = Cli::command();
            let man = clap_mangen::Man::new(clap_app);
            man.render(&mut io::stdout()).unwrap();
        }
    }
}
