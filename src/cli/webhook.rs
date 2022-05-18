use discord_webhook::client::WebhookClient;
use std::error::Error;

use crate::config::toml;

// send embed with link and deletehash to discord (something like logger)
pub async fn send_discord_webhook(
    link: &str,
    deletehash: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // get discord webhook uri from config
    let url = toml::parse().discord_webhook.uri;

    // create WebhookClient
    let client: WebhookClient = WebhookClient::new(&url);

    // get program version
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");

    // send discord webhook
    client
        .send(|message| {
            message.username("Imgurs").embed(|embed| {
                embed
                    .title(&link)
                    .description(&format!("Delete Hash ||{deletehash}||"))
                    .image(&link)
                    .footer(&format!("Imgurs v{version}"), None)
            })
        })
        .await?;

    Ok(())
}
