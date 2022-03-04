use std::error::Error;
use webhook::client::WebhookClient;

use crate::config::toml;

pub async fn send_discord_webhook(
    link: String,
    deletehash: String,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let url = toml::parse().discord_webhook.uri;
    let client: WebhookClient = WebhookClient::new(&url);

    let webhook = client
        .send(|message| {
            message.username("Imgurs").embed(|embed| {
                embed
                    .title("Imgurs")
                    .description(&format!("Delete Hash ||{deletehash}||"))
                    .image(&link)
                    .footer(
                        &format!(
                            "Imgurs v{}",
                            option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")
                        ),
                        None,
                    )
            })
        })
        .await;

    webhook
}
