use serde::{Deserialize, Serialize};

use super::{KakaAppInfo, KakaImage, KakaMedia};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct KakaCreative {
    #[serde(rename = "creativeType")]
    pub creative_type: i32,
    #[serde(rename = "interactionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<KakaImage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<KakaImage>,
    #[serde(rename = "targetUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink: Option<String>,
    #[serde(rename = "showUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_url: Option<Vec<String>>,
    #[serde(rename = "clickUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_url: Option<Vec<String>>,
    #[serde(rename = "dplSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpl_success: Option<Vec<String>>,
    #[serde(rename = "dplFail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpl_fail: Option<Vec<String>>,
    #[serde(rename = "downloadStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_start: Option<Vec<String>>,
    #[serde(rename = "downloadFinish")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_finish: Option<Vec<String>>,
    #[serde(rename = "installStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_start: Option<Vec<String>>,
    #[serde(rename = "installFinish")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_finish: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<String>,
    #[serde(rename = "winNoticeUrl")]
    pub win_notice_url: Vec<String>,
    #[serde(rename = "appInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_info: Option<KakaAppInfo>,
    #[serde(rename = "wechatAppletId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_applet_id: Option<String>,
    #[serde(rename = "wechatAppletPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_applet_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<KakaMedia>,
}
