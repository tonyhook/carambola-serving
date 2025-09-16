use serde::{Deserialize, Serialize};

use crate::client::spinview::{SpinviewApp, SpinviewVideo};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct SpinviewAd {
    pub ad_type: i32,
    pub interaction_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    pub width: i32,
    pub height: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_package: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<SpinviewApp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<SpinviewVideo>,
    pub click_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulk: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applet_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applet_path: Option<String>,
    pub impression_urls: Vec<String>,
    pub click_urls: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_begin_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_end_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_begin_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_end_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink_try_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink_success_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink_fail_urls: Option<Vec<String>>,
}
