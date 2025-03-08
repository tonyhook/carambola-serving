use serde::{Deserialize, Serialize};

use super::{ZhanqingExtendTracking, ZhanqingVideo};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct ZhanqingAd {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_area_report_url: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imgs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub action: i32,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkgname: Option<String>,
    pub exlist: Vec<String>,
    pub cklist: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nurl: Option<String>,
    #[serde(rename(deserialize = "loseNoticeUrls", serialize = "loseNoticeUrls"))]
    pub lose_notice_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exttracking: Option<ZhanqingExtendTracking>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appsize: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_ver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_developer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_privacy_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_permission: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_function_introduction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub videos: Option<ZhanqingVideo>,
}
