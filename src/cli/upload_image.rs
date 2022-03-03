use super::clipboard::set_clipboard;
use imgurs::api::{upload_image::upload_image as upload_img, ImgurClient};
use notify_rust::Notification;

use crate::{config::toml, cli::webhook::send_discord_webhook};

use super::print_image_info;

use base64::encode as base64_encode;
use std::{fs::read as fs_read, path::Path};

macro_rules! notify (
    ($notification: expr) => (
        if toml::parse().notification.enabled {
            $notification.show().expect("send notification");
        }
    );
);

pub async fn upload_image(client: ImgurClient, path: &str) {
    let mut image: String = path.to_string();

    if Path::new(path).exists() {
        let bytes = fs_read(path)
            .map_err(|err| err.to_string())
            .expect("read file");
        image = base64_encode(bytes);
    } else if !validator::validate_url(path) {
        panic!("{path} is not a url")
    }

    let i = upload_img(client, &image).await.unwrap_or_else(|err| {
        notify!(Notification::new()
            .summary("Error!")
            .body(&format!("Error: {}", &err.to_string()))
            .appname("Imgurs")); // I don't think you can set it to error

        panic!("{}", err)
    });
    print_image_info(i.clone());

    let body = format!("Uploaded {}", i.data.link);

    notify!(Notification::new().summary("Imgurs").body(&body));

    let config = toml::parse();

    if config.clipboard.enabled {
        set_clipboard(i.data.link.clone())
    }

    if config.discord_webhook.enabled {
        send_discord_webhook(i.data.link, i.data.deletehash.unwrap()).await.expect("send discord webhook");
    }
}
