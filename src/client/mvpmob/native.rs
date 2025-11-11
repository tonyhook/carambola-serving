use serde::{Deserialize, Serialize};

use super::{MvpmobIcon, MvpmobImage, MvpmobText, MvpmobTracker, MvpmobVideo};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MvpmobNative {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<MvpmobText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<MvpmobImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<MvpmobVideo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<MvpmobIcon>,
    pub winnotice_tracker: String,
    pub impression_trackers: Vec<String>,
    pub click_trackers: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_trackers: Option<Vec<MvpmobTracker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_area_report_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lurls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wurls: Option<Vec<String>>,
}
