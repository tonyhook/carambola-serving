use serde::{Deserialize, Serialize};

use super::{BillowlinkImage, BillowlinkMiniProgram, BillowlinkVideo};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct BillowlinkMaterialMeta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cta: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<BillowlinkImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<BillowlinkImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<BillowlinkImage>>,
    #[serde(rename(deserialize = "imageMode", serialize = "imageMode"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_mode: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<BillowlinkVideo>,
    #[serde(rename(deserialize = "htmlSnippet", serialize = "htmlSnippet"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_snippet: Option<String>,
    #[serde(rename(deserialize = "miniProgram", serialize = "miniProgram"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mini_program: Option<BillowlinkMiniProgram>,
}
