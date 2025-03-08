use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct ZhanqingPlaytrackers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmute: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fullscreen: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfullscreen: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upscroll: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downscroll: Option<Vec<String>>,
}
