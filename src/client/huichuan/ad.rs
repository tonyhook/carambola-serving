use serde::{Deserialize, Serialize};

use super::{HuichuanAdAction, HuichuanAdContent};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct HuichuanAd {
    pub ad_action: HuichuanAdAction,
    pub ad_content: HuichuanAdContent,
    pub style: String,
    pub ad_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i32>,
    pub turl: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wnurl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vurl: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curl: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub furl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme_feedback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eurl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_play_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lnurl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_area_report_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_type: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_time: Option<i64>,
}
