use serde::{Deserialize, Serialize};

use super::{RichmobMonitor, RichmobMonitorUrlVisit, RichmobVideo};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct RichmobAdv {
    #[serde(rename(deserialize = "slotId", serialize = "slotId"))]
    pub slot_id: String,
    #[serde(rename(deserialize = "imgUrls", serialize = "imgUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,
    #[serde(rename(deserialize = "winNotifyUrls", serialize = "winNotifyUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_notify_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "loseNotifyUrls", serialize = "loseNotifyUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lose_notify_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "clickAdUrl", serialize = "clickAdUrl"))]
    pub click_ad_url: String,
    #[serde(rename(deserialize = "creativeType", serialize = "creativeType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creative_type: Option<i32>,
    #[serde(rename(deserialize = "interactionType", serialize = "interactionType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink: Option<String>,
    #[serde(rename(deserialize = "universalLink", serialize = "universalLink"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universal_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename(deserialize = "appName", serialize = "appName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(rename(deserialize = "packageName", serialize = "packageName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(rename(deserialize = "appVersion", serialize = "appVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename(deserialize = "appIconUrl", serialize = "appIconUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_icon_url: Option<String>,
    #[serde(rename(deserialize = "appPrivacyUrls", serialize = "appPrivacyUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_privacy_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "appPermissionUrls", serialize = "appPermissionUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_permission_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "appDevComName", serialize = "appDevComName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_dev_com_name: Option<String>,
    #[serde(rename(deserialize = "appListUpload", serialize = "appListUpload"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_list_upload: Option<bool>,
    #[serde(rename(deserialize = "downloadUrl", serialize = "downloadUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(rename(deserialize = "iconSrcs", serialize = "iconSrcs"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_srcs: Option<String>,
    #[serde(rename(deserialize = "reportAddHeaderUa", serialize = "reportAddHeaderUa"))]
    pub report_add_header_ua: bool,
    #[serde(rename(deserialize = "reportCoordinatesInterger", serialize = "reportCoordinatesInterger"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_coordinates_interger: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<RichmobVideo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitors: Option<Vec<RichmobMonitor>>,
    #[serde(rename(deserialize = "monitorUrlVisit", serialize = "monitorUrlVisit"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_url_visit: Option<RichmobMonitorUrlVisit>,
}
