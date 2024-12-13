use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct FwbImgAsset {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imagetype: Option<i32>,
}
