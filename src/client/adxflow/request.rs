use serde::{Deserialize, Serialize};

use super::{AdxflowApp, AdxflowDevice, AdxflowImp, AdxflowUser};

#[derive(Serialize, Deserialize)]
pub struct AdxflowRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub token: String,
    pub app: AdxflowApp,
    pub device: AdxflowDevice,
    pub imp: Vec<AdxflowImp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<AdxflowUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcat: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badv: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmax: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<i32>,
}
