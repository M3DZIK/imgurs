use imgurs::api::{get_image::get_image, ImgurClient};

use super::print_image_info;

pub async fn image_info(client: ImgurClient, id: &str) {
    let i = get_image(client, id).await.expect("send api request");
    print_image_info(i);
}
