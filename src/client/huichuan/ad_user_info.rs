use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct HuichuanAdUserInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub res_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub res_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marriage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub school: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<u64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<Vec<f32>>,
}
