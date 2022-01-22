use crate::api::configuration::{api_url, ImgurHandle};

pub async fn delete_image(c: ImgurHandle, delete_hash: String) -> Result<String, String> {
    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

    let res = c
        .client
        .delete(api_url!(format!("image/{delete_hash}")))
        .header("Authorization", format!("Client-ID {}", c.client_id))
        .header(
            "User-Agent",
            format!("Imgur/{:?}", VERSION.unwrap_or("unknown")),
        )
        .send()
        .await
        .map_err(|err| err.to_string())?;

    let status = res.status();

    if status.is_client_error() || status.is_server_error() {
        let body = res.text().await.map_err(|err| err.to_string())?;
        Err(format!("server returned non-successful status code = {status}. body = {body}"))
    } else {
        Ok("If the delete hash was correct the image was deleted!".to_string())
    }
}
