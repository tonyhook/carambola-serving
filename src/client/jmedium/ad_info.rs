use serde::{Deserialize, Serialize};

use super::{JmediumAppAsset, JmediumImage, JmediumMiniProgram, JmediumTrack, JmediumVideo};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct JmediumAdInfo {
    #[serde(rename(deserialize = "requestId", serialize = "requestId"))]
    pub request_id: String,
    #[serde(rename(deserialize = "adType", serialize = "adType"))]
    pub ad_type: i32,
    #[serde(rename(deserialize = "interactionType", serialize = "interactionType"))]
    pub interaction_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(rename(deserialize = "adIcons", serialize = "adIcons"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_icons: Option<Vec<Option<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink: Option<String>,
    #[serde(rename(deserialize = "universalLink", serialize = "universalLink"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universal_link: Option<String>,
    #[serde(rename(deserialize = "landingPageUrl", serialize = "landingPageUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_page_url: Option<String>,
    #[serde(rename(deserialize = "kwaiLandingPageUrl", serialize = "kwaiLandingPageUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kwai_landing_page_url: Option<String>,
    #[serde(rename(deserialize = "downloadUrl", serialize = "downloadUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(rename(deserialize = "marketUrl", serialize = "marketUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_url: Option<String>,
    #[serde(rename(deserialize = "bidPrice", serialize = "bidPrice"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<i32>,
    #[serde(rename(deserialize = "winNoticeUrls", serialize = "winNoticeUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub win_notice_urls: Option<Vec<String>>,
    #[serde(rename(deserialize = "lossNoticeUrls", serialize = "lossNoticeUrls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loss_notice_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<JmediumAppAsset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<JmediumVideo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<JmediumImage>>,
    #[serde(rename(deserialize = "miniProgram", serialize = "miniProgram"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mini_program: Option<JmediumMiniProgram>,
    pub track: JmediumTrack,
}
