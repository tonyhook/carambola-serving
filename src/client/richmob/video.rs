use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct RichmobVideo {
    pub src: String,
    #[serde(rename(deserialize = "videoDuration", serialize = "videoDuration"))]
    pub video_duration: i32,
    pub size: i32,
    #[serde(rename(deserialize = "videoWidth", serialize = "videoWidth"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_width: Option<i32>,
    #[serde(rename(deserialize = "videoHeight", serialize = "videoHeight"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_height: Option<i32>,
    #[serde(rename(deserialize = "coverImgUrl", serialize = "coverImgUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_img_url: Option<Vec<String>>,
    #[serde(rename(deserialize = "buttonText", serialize = "buttonText"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_text: Option<String>,
    #[serde(rename(deserialize = "endImgUrl", serialize = "endImgUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_img_url: Option<Vec<String>>,
    #[serde(rename(deserialize = "endHtml", serialize = "endHtml"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_html: Option<String>,
    #[serde(rename(deserialize = "autoLanding", serialize = "autoLanding"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_landing: Option<bool>,
    #[serde(rename(deserialize = "prefetch", serialize = "prefetch"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<bool>,
    #[serde(rename(deserialize = "clickAble", serialize = "clickAble"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_able: Option<bool>,
    #[serde(rename(deserialize = "skipSeconds", serialize = "skipSeconds"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_seconds: Option<i32>,
}
