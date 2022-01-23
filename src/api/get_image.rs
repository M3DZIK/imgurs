use crate::api::configuration::{api_url, ImgurHandle};
use crate::api::ImageInfo;

pub async fn get_image(c: ImgurHandle, image: &str) -> Result<ImageInfo, String> {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

    let res = c
        .client
        .get(api_url!(format!("image/{image}")))
        .header("Authorization", format!("Client-ID {}", c.client_id))
        .header(
            "User-Agent",
            format!("Imgurs/{:?}", VERSION.unwrap_or("unknown")),
        )
        .send()
        .await
        .map_err(|err| err.to_string())?;

    let status = res.status();

    if status.is_client_error() || status.is_server_error() {
        Err(format!(
            "server returned non-successful status code = {status}."
        ))
    } else {
        let content: ImageInfo = res.json().await.map_err(|err| err.to_string())?;
        Ok(content)
    }
}
