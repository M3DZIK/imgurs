use serde::{Deserialize, Serialize};

/// Image Info Response
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ImageInfo {
    /// Image Data
    pub data: ImageInfoData,
    /// Request processed success or not.
    pub success: bool,
    /// HTTP status code from API request.
    pub status: i32,
}

/// Image Info Reponse (`data` json)
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ImageInfoData {
    /// Image ID
    /// e.g. `iDYNKJq`
    pub id: String,
    /// Image title
    pub title: Option<String>,
    /// Description of this image
    pub description: Option<String>,
    /// Image uploaded time
    pub datetime: i32,
    /// Image type
    /// e.g. `image/png`
    #[serde(rename = "type")]
    pub img_type: String,
    /// If image if animated (gif, etc)
    pub animated: bool,
    /// Width of this image
    pub width: i32,
    /// Height of this image
    pub height: i32,
    /// Image size in bytes
    pub size: i32,
    /// Unique image views
    pub views: i32,
    /// Bandwidth used by this image
    pub bandwidth: i64,
    /// If image is added to favorite
    pub favorite: bool,
    /// Delete hash (only show after image upload)
    pub deletehash: Option<String>,
    /// Link of this image
    pub link: String,
}
