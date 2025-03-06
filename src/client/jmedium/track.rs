use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct JmediumTrack {
    #[serde(rename(deserialize = "showUrls", serialize = "showUrls"))]
    pub show_urls: Vec<String>,
    #[serde(rename(deserialize = "clickUrls", serialize = "clickUrls"))]
    pub click_urls: Vec<String>,
    #[serde(rename(deserialize = "adLoadUrls", serialize = "adLoadUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_load_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "adSkipUrls", serialize = "adSkipUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_skip_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "adCloseUrls", serialize = "adCloseUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_close_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "wechatOpenUrls", serialize = "wechatOpenUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_open_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "startDownloadUrls", serialize = "startDownloadUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_download_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "finishDownloadUrls", serialize = "finishDownloadUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_download_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "pauseDownloadUrls", serialize = "pauseDownloadUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_download_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "continueDownloadUrls", serialize = "continueDownloadUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continue_download_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "deleteDownloadUrls", serialize = "deleteDownloadUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_download_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "startInstallUrls", serialize = "startInstallUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_install_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "finishInstallUrls", serialize = "finishInstallUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_install_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "activeAppUrls", serialize = "activeAppUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_app_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "deeplinkTryUrls", serialize = "deeplinkTryUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink_try_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "deeplinkSuccessUrls", serialize = "deeplinkSuccessUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink_success_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "deeplinkFailureUrls", serialize = "deeplinkFailureUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink_failure_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "deeplinkClickUrls", serialize = "deeplinkClickUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink_click_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "deeplinkInstalledkUrls", serialize = "deeplinkInstalledkUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink_installed_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "deeplinkUninstallkUrls", serialize = "deeplinkUninstallkUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink_uninstall_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoStartUrls", serialize = "videoStartUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_start_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoClickUrls", serialize = "videoClickUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_click_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoCompleteUrls", serialize = "videoCompleteUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_complete_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoFailUrls", serialize = "videoFailUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_fail_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoCloseUrls", serialize = "videoCloseUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_close_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoSkipUrls", serialize = "videoSkipUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_skip_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoPauseUrls", serialize = "videoPauseUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_pause_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoResumeUrls", serialize = "videoResumeUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_resume_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoReplayUrls", serialize = "videoReplayUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_replay_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoMuteUrls", serialize = "videoMuteUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_mute_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoUnmuteUrls", serialize = "videoUnmuteUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_unmute_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoFullscreenUrls", serialize = "videoFullscreenUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_fullscreen_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoExitFullscreenUrls", serialize = "videoExitFullscreenUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_exit_fullscreen_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoUpscrollUrls", serialize = "videoUpscrollUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_upscroll_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoDownscrollUrls", serialize = "videoDownscrollUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_downscroll_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoQuartileUrls", serialize = "videoQuartileUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quartile_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoHalfUrls", serialize = "videoHalfUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_half_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoThreeQuartileUrls", serialize = "videoThreeQuartileUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_three_quartile_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoPlay3sUrls", serialize = "videoPlay3sUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_play_3s_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoPlay5sUrls", serialize = "videoPlay5sUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_play_5s_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "clickAreaReportUrls", serialize = "clickAreaReportUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_area_report_urls: Option<String>,
}
