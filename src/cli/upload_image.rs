use imgurs::api::configuration::ImgurHandle;

use base64;
use log::{error, info};
use std::fs;
use std::path::Path;

use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{Duration, UNIX_EPOCH};

pub async fn upload_image(client: ImgurHandle, path: &str) {
    let mut image: String = path.to_string();

    if Path::new(path).exists() {
        let bytes = fs::read(path).map_err(|err| err.to_string()).unwrap();

        image = base64::encode(bytes);
    }

    match imgurs::api::upload_image::upload_image(client, &image).await {
        Ok(i) => {
            let d = UNIX_EPOCH + Duration::from_secs(i.data.datetime.try_into().unwrap());
            let datetime = DateTime::<Utc>::from(d);
            let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

            info!("ID           {}", i.data.id);
            info!("Upload Date  {} (UTC)", timestamp_str);
            info!("Type         {}", i.data.img_type);
            info!("Width        {}", i.data.width);
            info!("Height       {}", i.data.height);
            info!("File Size    {} KB", i.data.size / 1000);
            info!("Views        {}", i.data.views);
            info!("Bandwidth    {}", i.data.bandwidth);
            info!("Delete Hash  {}", i.data.deletehash);
            info!("Link         {}", i.data.link);
        }

        Err(e) => {
            error!("{e}");
        }
    }
}
