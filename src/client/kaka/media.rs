use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct KakaMedia {
    #[serde(rename = "type")]
    pub media_type: String,
    pub duration: i32,
    pub url: String,
    #[serde(rename = "middleUrl")]
    pub middle_url: String,
    pub width: i32,
    pub height: i32,
    #[serde(rename = "firstFrame")]
    pub first_frame: String,
}
