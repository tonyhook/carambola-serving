use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct FanglinVideo {
    pub video_url: String,
    pub video_size: i32,
    pub video_duration: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vd_start_tracks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vd_quar_tracks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vd_mid_tracks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vd_thd_tracks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vd_end_tracks: Option<Vec<String>>,
}
