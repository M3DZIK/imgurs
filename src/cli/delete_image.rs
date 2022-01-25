use imgurs::api::{configuration::ImgurHandle, delete_image::delete_image as del_img};

use log::{error, info};
use std::process::exit;

pub async fn delete_image(client: ImgurHandle, delete_hash: String) {
    let i = del_img(client, delete_hash).await.unwrap_or_else(|e| {
        error!("{e}");
        exit(1);
    });

    info!("{i}");
}
