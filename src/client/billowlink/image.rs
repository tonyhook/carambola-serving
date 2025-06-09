use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct BillowlinkImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,
    #[serde(rename(deserialize = "aspectRatio", serialize = "aspectRatio"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<f64>,
}
