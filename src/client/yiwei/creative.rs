use serde::{Deserialize, Serialize};

use super::{YiweiAppInfo, YiweiImage, YiweiMiniProgram, YiweiReward, YiweiTracker, YiweiVideo};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct YiweiCreative {
    pub creative_id: String,
    pub creative_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btn_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imgs: Option<Vec<YiweiImage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<YiweiVideo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<YiweiImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<YiweiAppInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mini_program: Option<YiweiMiniProgram>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward: Option<YiweiReward>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nurl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lurl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burl: Option<String>,
    pub trackers: Vec<YiweiTracker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub why_this_ad: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
}
