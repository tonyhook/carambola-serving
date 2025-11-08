use serde::{Deserialize, Serialize};

use super::HuoliMaterial;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct HuoliAd {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "creativeType", serialize = "creativeType"))]
    pub creative_type: Option<i32>,
    #[serde(rename(deserialize = "creativeId", serialize = "creativeId"))]
    pub creative_id: String,
    #[serde(rename(deserialize = "landingType", serialize = "landingType"))]
    pub landing_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<HuoliMaterial>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<HuoliMaterial>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename(deserialize = "clickUrl", serialize = "clickUrl"))]
    pub click_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "deeplinkUrl", serialize = "deeplinkUrl"))]
    pub deeplink_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulk: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "exposeTrackingUrl", serialize = "exposeTrackingUrl"))]
    pub expose_tracking_url: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "clickTrackingUrl", serialize = "clickTrackingUrl"))]
    pub click_tracking_url: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "dpTryUrl", serialize = "dpTryUrl"))]
    pub dp_try_url: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "dpSuccUrl", serialize = "dpSuccUrl"))]
    pub dp_succ_url: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "dpFailUrl", serialize = "dpFailUrl"))]
    pub dp_fail_url: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dealid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nurl: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fnurl: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wxoid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wxopath: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "appDownloadUrl", serialize = "appDownloadUrl"))]
    pub app_download_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "appName", serialize = "appName"))]
    pub app_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "appIcon", serialize = "appIcon"))]
    pub app_icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "appSize", serialize = "appSize"))]
    pub app_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "appVersion", serialize = "appVersion"))]
    pub app_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "appPackage", serialize = "appPackage"))]
    pub app_package: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "appDeveloper", serialize = "appDeveloper"))]
    pub app_developer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "appDescriptioin", serialize = "appDescriptioin"))]
    pub app_descriptioin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "appDescriptioinUrl", serialize = "appDescriptioinUrl"))]
    pub app_descriptioin_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "appPrivacyUrl", serialize = "appPrivacyUrl"))]
    pub app_privacy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "appPermissionUrl", serialize = "appPermissionUrl"))]
    pub app_permission_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "downloadStart", serialize = "downloadStart"))]
    pub download_start: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "downloadEnd", serialize = "downloadEnd"))]
    pub download_end: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "installStart", serialize = "installStart"))]
    pub install_start: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "installEnd", serialize = "installEnd"))]
    pub install_end: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "activateApp", serialize = "activateApp"))]
    pub activate_app: Option<Vec<String>>,
}
