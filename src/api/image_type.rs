use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ImageInfo {
    pub data: ImageInfoData,
    pub success: bool,
    pub status: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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
