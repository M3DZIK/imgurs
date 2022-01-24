use super::print_image_info;
use imgurs::api::configuration::ImgurHandle;

use base64;
use log::error;
use std::fs;
use std::path::Path;

pub async fn upload_image(client: ImgurHandle, path: &str) {
    let mut image: String = path.to_string();

    if Path::new(path).exists() {
        let bytes = fs::read(path).map_err(|err| err.to_string()).unwrap();

        image = base64::encode(bytes);
    }

    match imgurs::api::upload_image::upload_image(client, &image).await {
        Ok(i) => print_image_info(i, true),
        Err(e) => error!("{e}")
    }
}
