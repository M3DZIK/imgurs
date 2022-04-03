use colored::Colorize;
use imgurs::ImgurClient;

pub async fn delete_image(client: ImgurClient, delete_hash: String) {
    // delete image from imgur
    client
        .delete_image(delete_hash)
        .await
        .expect("send api request");

    println!(
        "{}",
        "If Delete Hash was correct the image was deleted!".magenta()
    );
}
