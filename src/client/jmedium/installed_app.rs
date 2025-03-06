use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct JmediumInstalledApp {
    #[serde(rename(deserialize = "appName", serialize = "appName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(rename(deserialize = "pkgName", serialize = "pkgName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkg_name: Option<String>,
    #[serde(rename(deserialize = "appVersion", serialize = "appVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename(deserialize = "appVersionCode", serialize = "appVersionCode"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version_code: Option<String>,
    #[serde(rename(deserialize = "systemApp", serialize = "systemApp"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_app: Option<i32>,
    #[serde(rename(deserialize = "firstInstallTime", serialize = "firstInstallTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_install_time: Option<i64>,
    #[serde(rename(deserialize = "lastUpdateTime", serialize = "lastUpdateTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<i64>,
}
