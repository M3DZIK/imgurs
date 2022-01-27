use imgurs::api::{delete_image::delete_image as del_img, ImgurClient};

use log::info;

pub async fn delete_image(client: ImgurClient, delete_hash: String) {
    let i = del_img(client, delete_hash)
        .await
        .expect("send api request");
    info!("{i}");
}
