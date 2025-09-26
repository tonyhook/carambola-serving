use serde::{Deserialize, Serialize};

use super::KakaCreative;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct KakaAd {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "impId")]
    pub imp_id: String,
    pub creative: KakaCreative,
    #[serde(rename = "bidPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<i32>,
}
