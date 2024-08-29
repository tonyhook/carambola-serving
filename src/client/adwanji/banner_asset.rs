use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct AdwanjiBannerAsset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimes: Option<String>,
    pub iurl: String,
}
