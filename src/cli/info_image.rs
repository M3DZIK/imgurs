use imgurs::api::{configuration::ImgurHandle, get_image::get_image};

use super::print_image_info;

use base64;
use log::error;
use std::fs;
use std::path::Path;

pub async fn image_info(client: ImgurHandle, path: &str) {
    let mut image: String = path.to_string();

    if Path::new(path).exists() {
        let bytes = fs::read(path).map_err(|err| err.to_string()).unwrap();

        image = base64::encode(bytes);
    }

    match get_image(client, &image).await {
        Ok(i) => print_image_info(i, false),
        Err(e) => error!("{e}"),
    }
}
