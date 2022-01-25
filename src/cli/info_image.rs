use imgurs::api::{configuration::ImgurHandle, get_image::get_image};

use super::print_image_info;

use log::error;
use std::process::exit;

pub async fn image_info(client: ImgurHandle, id: &str) {
    let i = get_image(client, id).await.unwrap_or_else(|e| {
        error!("{e}");
        exit(1);
    });

    print_image_info(i, false);
}
