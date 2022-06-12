use serde_derive::{Deserialize, Serialize};

/// Image Info Response
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageInfo {
    pub data: ImageInfoData,
    pub success: bool,
    pub status: i32,
}

/// Image Info Reponse (`data` json)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageInfoData {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub datetime: i32,
    #[serde(rename = "type")]
    pub img_type: String,
    pub animated: bool,
    pub width: i32,
    pub height: i32,
    pub size: i32,
    pub views: i32,
    pub bandwidth: i64,
    pub favorite: bool,
    pub deletehash: Option<String>,
    pub link: String,
}
