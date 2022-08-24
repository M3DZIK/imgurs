mod clipboard;
mod credits;
mod delete_image;
mod info_image;
mod upload_image;

pub use self::{clipboard::*, credits::*, delete_image::*, info_image::*, upload_image::*};

use chrono::{prelude::DateTime, Utc};
use colored::Colorize;
use imgurs::ImageInfo;
use std::time::{Duration, UNIX_EPOCH};

// print image information from imgur
pub fn print_image_info(i: &ImageInfo) {
    // format image upload date
    let d = UNIX_EPOCH + Duration::from_secs(i.data.datetime.try_into().unwrap());
    let datetime = DateTime::<Utc>::from(d);
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    // image title
    if i.data.title != None {
        let title = i.data.title.clone();

        println!(
            "{} {}",
            "title".green(),
            title.unwrap_or_else(|| "unknown".to_string()).magenta()
        );
    }

    // image description
    if i.data.description != None {
        let desc = i.data.description.clone();

        println!(
            "{} {}",
            "description".green(),
            desc.unwrap_or_else(|| "unknown".to_string()).magenta()
        );
    }

    // image deletehas
    if i.data.deletehash != None {
        let delhash = i.data.deletehash.clone();

        println!(
            "{} {}",
            "deletehash".green(),
            delhash.unwrap_or_else(|| "unknown".to_string()).magenta()
        );
    }

    println!("{} {}", "id".green(), i.data.id.magenta());
    println!(
        "{} {} {}",
        "upload date".green(),
        timestamp_str.magenta(),
        "(UTC)".blue()
    );
    println!("{} {}", "type".green(), i.data.img_type.magenta());
    println!("{} {}", "width".green(), i.data.width.to_string().magenta());
    println!(
        "{} {}",
        "height".green(),
        i.data.height.to_string().magenta()
    );
    println!(
        "{} {} {}",
        "size".green(),
        (i.data.size / 1000).to_string().magenta(),
        "KB".blue()
    );
    println!("{} {}", "views".green(), i.data.views.to_string().magenta());
    println!(
        "{} {}",
        "bandwidth".green(),
        i.data.bandwidth.to_string().magenta()
    );
    println!("{} {}", "link".green(), i.data.link.magenta());
}
