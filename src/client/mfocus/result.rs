use serde::{Deserialize, Serialize};

use super::{MfocusAdContent, MfocusWechatMiniProgram};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MfocusResult {
    pub adid: String,
    pub clk: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink: Option<String>,
    #[serde(rename(deserialize = "imprTrackers", serialize = "imprTracker"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impr_trackers: Option<Vec<String>>,
    #[serde(rename(deserialize = "clkTrackers", serialize = "clkTrackers"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clk_trackers: Option<Vec<String>>,
    #[serde(rename(deserialize = "wxMiniProgram", serialize = "wxMiniProgram"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wx_mini_program: Option<MfocusWechatMiniProgram>,
    #[serde(rename(deserialize = "adContent", serialize = "adContent"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_content: Option<MfocusAdContent>,
    #[serde(rename(deserialize = "styleName", serialize = "styleName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpm: Option<i32>,
}
