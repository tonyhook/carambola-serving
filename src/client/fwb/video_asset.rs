use serde::{Deserialize, Serialize};

use super::FwbCard;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct FwbVideoAsset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videotype: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimes: Option<Vec<String>>,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_min_time: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_load_ttl: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<FwbCard>,
}
