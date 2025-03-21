use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct HuichuanAdAppInfo {
    pub fr: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utdid: Option<String>,
    pub ua: String,
    pub pkg_name: String,
    pub pkg_ver: String,
    pub app_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_app_ids: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ssl: Option<String>,
    pub category: Vec<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}
