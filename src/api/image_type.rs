use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageInfo {
    pub data: ImageInfoData,
}

#[derive(Debug, Serialize, Deserialize)]
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
