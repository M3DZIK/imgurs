pub mod credits;
pub mod delete_image;
pub mod info_image;
pub mod parse;
pub mod upload_image;

use chrono::prelude::DateTime;
use chrono::Utc;
use log::{error, info};
use std::time::{Duration, UNIX_EPOCH};

use imgurs::api::ImageInfo;

use notify_rust::Notification;
use std::process::exit;

pub fn print_image_info(i: ImageInfo, notify: bool) {
    let d = UNIX_EPOCH + Duration::from_secs(i.data.datetime.try_into().unwrap());
    let datetime = DateTime::<Utc>::from(d);
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    if i.data.title != None {
        info!(
            "Title        {}",
            i.data.title.unwrap_or_else(|| "unknown".to_string())
        );
    }
    if i.data.description != None {
        info!(
            "Description  {}",
            i.data.description.unwrap_or_else(|| "unknown".to_string())
        );
    }
    if i.data.deletehash != None {
        info!(
            "Deletehash   {}",
            i.data.deletehash.unwrap_or_else(|| "unknown".to_string())
        );
    }

    info!("ID           {}", i.data.id);
    info!("Upload Date  {} (UTC)", timestamp_str);
    info!("Type         {}", i.data.img_type);
    info!("Width        {}", i.data.width);
    info!("Height       {}", i.data.height);
    info!("File Size    {} KB", i.data.size / 1000);
    info!("Views        {}", i.data.views);
    info!("Bandwidth    {}", i.data.bandwidth);
    info!("Link         {}", i.data.link);

    if notify {
        Notification::new()
            .summary("Imgurs")
            .body(&format!("Uploaded {}", i.data.link))
            .show()
            .unwrap_or_else(|e| {
                error!("send notification: {}", e);

                exit(2)
            });
    }
}
