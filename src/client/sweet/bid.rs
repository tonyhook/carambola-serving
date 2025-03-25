use serde::{Deserialize, Serialize};

use super::{SweetAppAsset, SweetTracker, SweetVideo};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct SweetBid {
    #[serde(rename(deserialize = "bidId", serialize = "bidId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_id: Option<String>,
    #[serde(rename(deserialize = "tagId", serialize = "tagId"))]
    pub tag_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(rename(deserialize = "iconUrl", serialize = "iconUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename(deserialize = "imgUrls", serialize = "imgUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,
    #[serde(rename(deserialize = "landingUrl", serialize = "landingUrl"))]
    pub landing_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink: Option<String>,
    #[serde(rename(deserialize = "universalLink", serialize = "universalLink"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universal_link: Option<String>,
    #[serde(rename(deserialize = "downloadUrl", serialize = "downloadUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(rename(deserialize = "bidFloor", serialize = "bidFloor"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_floor: Option<i32>,
    #[serde(rename(deserialize = "winUrls", serialize = "winUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "loseUrls", serialize = "loseUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lose_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "cType", serialize = "cType"))]
    pub c_type: i32,
    #[serde(rename(deserialize = "ciType", serialize = "ciType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ci_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<SweetAppAsset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<SweetVideo>,
    pub trackers: Vec<SweetTracker>,
    #[serde(rename(deserialize = "clickAreaReportUrls", serialize = "clickAreaReportUrls"))]
    pub click_area_report_urls: Option<Vec<String>>,
}
