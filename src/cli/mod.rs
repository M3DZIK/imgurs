pub mod clipboard;
pub mod credits;
pub mod delete_image;
pub mod info_image;
pub mod parse;
pub mod upload_image;

use imgurs::api::ImageInfo;

use chrono::{prelude::DateTime, Utc};
use colored::Colorize;
use std::time::{Duration, UNIX_EPOCH};

pub fn print_image_info(i: ImageInfo) {
    let d = UNIX_EPOCH + Duration::from_secs(i.data.datetime.try_into().unwrap());
    let datetime = DateTime::<Utc>::from(d);
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    if i.data.title != None {
        println!(
            "{} {}",
            "title".green(),
            i.data
                .title
                .unwrap_or_else(|| "unknown".to_string())
                .magenta()
        );
    }
    if i.data.description != None {
        println!(
            "{} {}",
            "description".green(),
            i.data
                .description
                .unwrap_or_else(|| "unknown".to_string())
                .magenta()
        );
    }
    if i.data.deletehash != None {
        println!(
            "{} {}",
            "deletehash".green(),
            i.data
                .deletehash
                .unwrap_or_else(|| "unknown".to_string())
                .magenta()
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
