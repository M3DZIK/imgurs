use imgurs::api::{upload_image::upload_image as upload_img, ImgurClient};

use super::print_image_info;

use base64::encode as base64_encode;
use std::{fs::read as fs_read, path::Path};

pub async fn upload_image(client: ImgurClient, path: &str) {
    let mut image: String = path.to_string();

    if Path::new(path).exists() {
        let bytes = fs_read(path)
            .map_err(|err| err.to_string())
            .expect("read file");
        image = base64_encode(bytes);
    }

    let i = upload_img(client, &image).await.expect("send api request");
    print_image_info(i, true);
}
