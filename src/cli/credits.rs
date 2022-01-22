use imgurs::api::configuration::ImgurHandle;
use imgurs::api::rate_limit::*;

use log::{info, error};

use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{UNIX_EPOCH, Duration};

pub async fn credits(client: ImgurHandle) {
    match rate_limit(client).await {
        Ok(i) => {
            let d = UNIX_EPOCH + Duration::from_secs(i.data.user_reset.try_into().unwrap());
            let datetime = DateTime::<Utc>::from(d);
            let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

            info!("User Limit         {}", i.data.user_limit);
            info!("User Remaining     {}", i.data.user_remaining);
            info!("User Reset         {} (UTC)", timestamp_str);
            info!("Client Limit       {}", i.data.client_limit);
            info!("Client Remaining   {}", i.data.client_remaining);
        }

        Err(e) => {
            error!("{}", e);
        }
    }
}
