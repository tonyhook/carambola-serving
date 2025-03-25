use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct SweetVideo {
    pub url: String,
    pub duration: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vw: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vh: Option<i32>,
    #[serde(rename(deserialize = "coverImgUrl", serialize = "coverImgUrl"))]
    pub cover_img_url: Vec<String>,
    #[serde(rename(deserialize = "buttonText", serialize = "buttonText"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_text: Option<String>,
    #[serde(rename(deserialize = "endImgUrl", serialize = "endImgUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_img_url: Option<String>,
    #[serde(rename(deserialize = "endHtml", serialize = "endHtml"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_html: Option<String>,
    #[serde(rename(deserialize = "skipSec", serialize = "skipSec"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_sec: Option<i32>,
    #[serde(rename(deserialize = "comLanding", serialize = "comLanding"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub com_landing: Option<bool>,
    #[serde(rename(deserialize = "proLanding", serialize = "proLanding"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pro_landing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<bool>,
}
