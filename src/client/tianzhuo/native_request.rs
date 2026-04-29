use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TianzhuoNativeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battr: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_nums: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_field: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_max: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc_max: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iw: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ih: Option<i32>,
}
