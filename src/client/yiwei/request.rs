use serde::{Deserialize, Serialize};

use super::{YiweiApp, YiweiDevice, YiweiExt, YiweiImp, YiweiUser, YiweiWebSite};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct YiweiRequest {
    pub request_id: String,
    pub protocol_version: String,
    pub app: YiweiApp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_site: Option<YiweiWebSite>,
    pub device: YiweiDevice,
    pub imps: Vec<YiweiImp>,
    pub timestamp: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<YiweiUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencies: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<Vec<YiweiExt>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_test: Option<i32>,
}
