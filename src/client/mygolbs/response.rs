use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MygolbsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(rename(deserialize = "mediaStyle", serialize = "mediaStyle"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_style: Option<i32>,
    #[serde(rename(deserialize = "appName", serialize = "appName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(rename(deserialize = "compName", serialize = "compName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comp_name: Option<String>,
    #[serde(rename(deserialize = "versionName", serialize = "versionName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
    #[serde(rename(deserialize = "secretUrl", serialize = "secretUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_url: Option<String>,
    #[serde(rename(deserialize = "permissionUrl", serialize = "permissionUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(rename(deserialize = "appIcon", serialize = "appIcon"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_icon: Option<String>,
    #[serde(rename(deserialize = "descriptionUrl", serialize = "descriptionUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pics: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(rename(deserialize = "deepLink", serialize = "deepLink"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deep_link: Option<String>,
    #[serde(rename(deserialize = "wxMiniProId", serialize = "wxMiniProId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wx_mini_pro_id: Option<String>,
    #[serde(rename(deserialize = "wxMiniProPath", serialize = "wxMiniProPath"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wx_mini_pro_path: Option<String>,
    #[serde(rename(deserialize = "adMark", serialize = "adMark"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_mark: Option<String>,
    #[serde(rename(deserialize = "universalLink", serialize = "universalLink"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universal_link: Option<String>,
    #[serde(rename(deserialize = "unfoldMonitorLink", serialize = "unfoldMonitorLink"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfold_monitor_link: Option<Vec<String>>,
    #[serde(rename(deserialize = "clickMonitorLink", serialize = "clickMonitorLink"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_monitor_link: Option<Vec<String>>,
    #[serde(rename(deserialize = "dwsUrls", serialize = "dwsUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dws_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "dweUrls", serialize = "dweUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dwe_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "instBUrls", serialize = "instBUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_b_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "instUrls", serialize = "instUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "dnUrls", serialize = "dnUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dn_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "dplinkUrls", serialize = "dplinkUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dplink_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "dplinkTryUrls", serialize = "dplinkTryUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dplink_try_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "dplinkErrUrls", serialize = "dplinkErrUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dplink_err_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "adShowType", serialize = "adShowType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_show_type: Option<i32>,
    #[serde(rename(deserialize = "videoUrl", serialize = "videoUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename(deserialize = "mimeType", serialize = "mimeType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename(deserialize = "coverUrl", serialize = "coverUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
    #[serde(rename(deserialize = "videoStartUrls", serialize = "videoStartUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_start_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoFirstQuartileUrls", serialize = "videoFirstQuartileUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_first_quartile_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoMidpointUrls", serialize = "videoMidpointUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_midpoint_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoThirdQuartileUrls", serialize = "videoThirdQuartileUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_third_quartile_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoEndUrls", serialize = "videoEndUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_end_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "videoFailUrls", serialize = "videoFailUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_fail_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clk: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmute: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspend: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsuspend: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lurl: Option<Vec<String>>,
    #[serde(rename(deserialize = "winNoticeUrl", serialize = "winNoticeUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_notice_url: Option<Vec<String>>,
}
