use imgurs::api::{rate_limit::rate_limit, ImgurHandle};

use chrono::{prelude::DateTime, Utc};
use log::{error, info};
use std::{
    process::exit,
    time::{Duration, UNIX_EPOCH},
};

pub async fn credits(client: ImgurHandle) {
    let i = rate_limit(client).await.unwrap_or_else(|e| {
        error!("{e}");
        exit(1);
    });

    let d = UNIX_EPOCH + Duration::from_secs(i.data.user_reset.try_into().unwrap());
    let datetime = DateTime::<Utc>::from(d);
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

    info!("User Limit         {}", i.data.user_limit);
    info!("User Remaining     {}", i.data.user_remaining);
    info!("User Reset         {} (UTC)", timestamp_str);
    info!("Client Limit       {}", i.data.client_limit);
    info!("Client Remaining   {}", i.data.client_remaining);
}
