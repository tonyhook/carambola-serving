use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct ImageAssetFormat {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imagetype: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wmin: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hmin: Option<i32>,
}
