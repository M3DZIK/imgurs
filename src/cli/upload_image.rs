use super::clipboard::set_clipboard;
use imgurs::ImgurClient;
use notify_rust::Notification;

use crate::{cli::webhook::send_discord_webhook, config::toml};

use super::print_image_info;

// show notification
macro_rules! notify (
    ($notification: expr) => (
        if toml::parse().notification.enabled {
            $notification.show().expect("send notification");
        }
    );
);

pub async fn upload_image(client: ImgurClient, path: String) {
    // parse configuration file
    let config = toml::parse();

    // upload a image to imgur
    let mut i = client.upload_image(path).await.unwrap_or_else(|err| {
        notify!(Notification::new()
            .summary("Error!")
            .body(&format!("Error: {}", &err.to_string()))
            .appname("Imgurs")); // I don't think you can set it to error

        panic!("send request to imagur api: {}", err)
    });

    // change domain to proxy (to be set in config)
    if config.imgur.image_cdn != "i.imgur.com" {
        i.data.link = i.data.link.replace("i.imgur.com", &config.imgur.image_cdn)
    }

    // print image information from imgur
    print_image_info(i.clone());

    // send notification that the image has been uploaded
    notify!(Notification::new()
        .summary("Imgurs")
        .body(&format!("Uploaded {}", i.data.link)));

    // if enabled copy link to clipboard
    if config.clipboard.enabled {
        set_clipboard(i.data.link.clone())
    }

    // if enabled send embed with link and deletehash to discord (something like logger)
    if config.discord_webhook.enabled {
        send_discord_webhook(i.data.link, i.data.deletehash.unwrap())
            .await
            .expect("send discord webhook");
    }
}
