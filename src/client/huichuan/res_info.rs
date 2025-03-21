use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct HuichuanResInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub res_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub res_title: Option<String>,
}
