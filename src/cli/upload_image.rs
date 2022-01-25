use super::print_image_info;
use imgurs::api::{configuration::ImgurHandle, upload_image::upload_image as upload_img};

use base64::encode as base64_encode;
use log::error;
use std::{fs::read as fs_read, path::Path, process::exit};

pub async fn upload_image(client: ImgurHandle, path: &str) {
    let mut image: String = path.to_string();

    if Path::new(path).exists() {
        let bytes = fs_read(path).map_err(|err| err.to_string()).unwrap();

        image = base64_encode(bytes);
    }

    let i = upload_img(client, &image).await.unwrap_or_else(|e| {
        error!("{e}");
        exit(1);
    });

    print_image_info(i, true);
}
