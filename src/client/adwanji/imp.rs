use serde::{Deserialize, Serialize};

use super::{AdwanjiBannerFormat, AdwanjiFeedFormat, AdwanjiVideoFormat};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct AdwanjiImp {
    pub slotid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<AdwanjiBannerFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feed: Option<AdwanjiFeedFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<AdwanjiVideoFormat>,
    pub support_deeplink: i32,
    pub support_universal: i32,
    #[serde(rename(deserialize = "bidPrice", serialize = "bidPrice"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<i32>,
}
