use imgurs::api::configuration::ImgurHandle;
use imgurs::api;

use log::{info, error};

pub async fn delete_image(client: ImgurHandle, delete_hash: String) {
    match api::delete_image::delete_image(client, delete_hash).await {
        Ok(i) => {
            info!("{i}")
        }

        Err(e) => {
            error!("{}", e);
        }
    }
}
