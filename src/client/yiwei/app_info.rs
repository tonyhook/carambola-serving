use serde::{Deserialize, Serialize};

use super::{YiweiPermission};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct YiweiAppInfo {
    pub app_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkg_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub itunes_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkg_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introduce_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<YiweiPermission>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icp_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suitable_age: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}
