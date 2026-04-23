use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct AdwanjiVideoAsset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,
    pub iurl: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reward_duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_w: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_h: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_screen: Option<i32>,
    pub clickable: i32,
    pub is_auto_langding: i32,
}
