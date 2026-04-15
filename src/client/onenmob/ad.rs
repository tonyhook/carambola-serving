use serde::{Deserialize, Serialize};

use super::{OnenmobTrack, OnenmobVideo};

#[derive(Serialize, Deserialize)]
pub struct OnenmobAd {
    #[serde(rename(deserialize = "adId", serialize = "adId"))]
    pub ad_id: String,
    #[serde(rename(deserialize = "imageSrcs", serialize = "imageSrcs"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_srcs: Option<Vec<String>>,
    #[serde(rename(deserialize = "showUrl", serialize = "showUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_url: Option<Vec<String>>,
    #[serde(rename(deserialize = "clickUrl", serialize = "clickUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_url: Option<Vec<String>>,
    #[serde(rename(deserialize = "clickAdUrl", serialize = "clickAdUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_ad_url: Option<String>,
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
    #[serde(rename(deserialize = "appDownloadUrl", serialize = "appDownloadUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_download_url: Option<String>,
    #[serde(rename(deserialize = "permissionUrl", serialize = "permissionUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_url: Option<String>,
    #[serde(rename(deserialize = "privacyPolicyUrl", serialize = "privacyPolicyUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<String>,
    #[serde(rename(deserialize = "developerName", serialize = "developerName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub developer_name: Option<String>,
    #[serde(rename(deserialize = "introUrl", serialize = "introUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro_url: Option<String>,
    #[serde(rename(deserialize = "downloadUrl", serialize = "downloadUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(rename(deserialize = "iconSrcs", serialize = "iconSrcs"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_srcs: Option<String>,
    #[serde(rename(deserialize = "reportAddHeaderUa", serialize = "reportAddHeaderUa"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_add_header_ua: Option<bool>,
    #[serde(rename(deserialize = "reportCoordinatesInterger", serialize = "reportCoordinatesInterger"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_coordinates_interger: Option<bool>,
    #[serde(rename(deserialize = "reportPixelFlag", serialize = "reportPixelFlag"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_pixel_flag: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<OnenmobVideo>,
    pub tracks: Vec<OnenmobTrack>,
}
