use chrono::{prelude::DateTime, Utc};
use colored::Colorize;
use imgurs::ImgurClient;
use std::time::{Duration, UNIX_EPOCH};

pub async fn credits(client: ImgurClient) {
    // get client ratelimit from imgur api
    let i = client
        .rate_limit()
        .await
        .expect("send request to imgur api");

    // format image upload date
    let date = UNIX_EPOCH + Duration::from_secs(i.data.user_reset.try_into().unwrap());
    let datetime = DateTime::<Utc>::from(date);
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    println!(
        "{} {}",
        "user limit".green(),
        i.data.user_limit.to_string().magenta()
    );
    println!(
        "{} {}",
        "user remaining".green(),
        i.data.user_remaining.to_string().magenta()
    );
    println!(
        "{} {} {}",
        "user reset".green(),
        timestamp_str.magenta(),
        "(UTC)".blue()
    );
    println!(
        "{} {}",
        "client limit".green(),
        i.data.client_limit.to_string().magenta()
    );
    println!(
        "{} {}",
        "client remaining ".green(),
        i.data.client_remaining.to_string().magenta()
    );
}
