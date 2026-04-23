use serde::{Deserialize, Serialize};

use super::{AdwanjiAppAsset, AdwanjiBannerAsset, AdwanjiFeedAsset, AdwanjiVideoAsset, AdwanjiEvents};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct AdwanjiBid {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adid: Option<String>,
    #[serde(rename(deserialize = "landingUrl", serialize = "landingUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_url: Option<String>,
    #[serde(rename(deserialize = "downloadUrl", serialize = "downloadUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(rename(deserialize = "downloadType", serialize = "downloadType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shake: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wxappid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wxapppath: Option<String>,
    #[serde(rename(deserialize = "iosAppId", serialize = "iosAppId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios_app_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<AdwanjiAppAsset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<AdwanjiBannerAsset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<AdwanjiFeedAsset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<AdwanjiVideoAsset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<AdwanjiEvents>,
    #[serde(rename(deserialize = "universalUrl", serialize = "universalUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub universal_url: Option<String>,
    #[serde(rename(deserialize = "marketUrl", serialize = "marketUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i32>,
}
