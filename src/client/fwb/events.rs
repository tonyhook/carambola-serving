use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct FwbEvents {
    pub imp_urls: Vec<String>,
    pub click_urls: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_dod_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_dod_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_install_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_install_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_play_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_25play_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_50play_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_75play_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_play_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_play_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink_pre_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink_furls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_play_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute_play_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_play_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_play_urls: Option<Vec<String>>,
}
