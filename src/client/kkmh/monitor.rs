use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct KkmhMonitor {
    pub exposal_urls: Vec<String>,
    pub clk_urls: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dn_begin_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dn_completed_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_begin_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_completed_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_launch_urls: Option<Vec<String>>,
}
