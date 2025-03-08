use serde::{Deserialize, Serialize};

use super::{ZhanqingPlaypercentage, ZhanqingPlaytrackers};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct ZhanqingVideo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_loaded_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_play_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_play_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_close: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_skip: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_exposure_tracking: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_clos_tracking: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playpercentages: Option<Vec<ZhanqingPlaypercentage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_1_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_2_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_3_trackers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_trackers: Option<ZhanqingPlaytrackers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_card_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_card_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_button_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_url: Option<String>,
}
