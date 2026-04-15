use serde::{Deserialize, Serialize};

use super::AdxflowAdm;

#[derive(Serialize, Deserialize)]
pub struct AdxflowBid {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_area_report_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nurl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adm: Option<AdxflowAdm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imp_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub down_start_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub down_comp_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_start_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_comp_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink_start_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink_comp_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink_fail_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_start_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_comp_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_pause_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_resume_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_skip_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_full_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_ext_full_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_close_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_mute_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_unmute_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quarter_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_half_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_threefourhs_trackers: Option<Vec<String>>,
}
