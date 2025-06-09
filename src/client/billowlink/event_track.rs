use serde::{Deserialize, Serialize};

use super::BillowlinkProgressTrack;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct BillowlinkEventTrack {
    #[serde(rename(deserialize = "impTracks", serialize = "impTracks"))]
    pub imp_tracks: Vec<String>,
    #[serde(rename(deserialize = "clkTracks", serialize = "clkTracks"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clk_tracks: Option<Vec<String>>,
    #[serde(rename(deserialize = "appUninstalled", serialize = "appUninstalled"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_uninstalled: Option<Vec<String>>,
    #[serde(rename(deserialize = "appInstalled", serialize = "appInstalled"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_installed: Option<Vec<String>>,
    #[serde(rename(deserialize = "dplTry", serialize = "dplTry"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpl_try: Option<Vec<String>>,
    #[serde(rename(deserialize = "dplSuccess", serialize = "dplSuccess"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpl_success: Option<Vec<String>>,
    #[serde(rename(deserialize = "dplFailed", serialize = "dplFailed"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpl_failed: Option<Vec<String>>,
    #[serde(rename(deserialize = "fallbackTracks", serialize = "fallbackTracks"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_tracks: Option<Vec<String>>,
    #[serde(rename(deserialize = "dlTracks", serialize = "dlTracks"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dl_tracks: Option<Vec<String>>,
    #[serde(rename(deserialize = "installTracks", serialize = "installTracks"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_tracks: Option<Vec<String>>,
    #[serde(rename(deserialize = "dlStartTracks", serialize = "dlStartTracks"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dl_start_tracks: Option<Vec<String>>,
    #[serde(rename(deserialize = "installStartTracks", serialize = "installStartTracks"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_start_tracks: Option<Vec<String>>,
    #[serde(rename(deserialize = "gdtTracks", serialize = "gdtTracks"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gdt_tracks: Option<i32>,
    #[serde(rename(deserialize = "progressTracks", serialize = "progressTracks"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_tracks: Option<BillowlinkProgressTrack>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoplay: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoPause", serialize = "videoPause"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_pause: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoResume", serialize = "videoResume"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_resume: Option<Vec<String>>,
    #[serde(rename(deserialize = "skipTracks", serialize = "skipTracks"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_tracks: Option<Vec<String>>,
    #[serde(rename(deserialize = "stopTracks", serialize = "stopTracks"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_tracks: Option<Vec<String>>,
}
