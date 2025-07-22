use serde::{Deserialize, Serialize};

use super::{LeidongIcon, LeidongImage, LeidongText, LeidongTracker, LeidongVideo};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct LeidongNative {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<LeidongText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<LeidongImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<LeidongVideo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<LeidongIcon>,
    pub winnotice_tracker: String,
    pub impression_trackers: Vec<String>,
    pub click_trackers: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_trackers: Option<Vec<LeidongTracker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_area_report_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lnurl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks_lnurl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ks_winnotice_tracker: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lnurl_list: Option<Vec<String>>,
}
