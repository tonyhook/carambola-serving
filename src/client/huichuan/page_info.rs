use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct HuichuanPageInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_url: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_title: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_refer: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_kw: Option<String>,
}
