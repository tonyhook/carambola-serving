use serde::{Deserialize, Serialize};

use super::FanglinVideo;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct FanglinAd {
    pub pid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imgs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icons: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_logo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creative_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_bundle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mini_program_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mini_program_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<FanglinVideo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imp_tracks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clk_tracks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dn_start_tracks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dn_succ_tracks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_start_tracks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_succ_tracks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ap_start_tracks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dp_try_tracks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dp_succ_tracks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dp_err_tracks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecpm: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_notice_url: Option<String>,
}
