use serde::{Deserialize, Serialize};

use super::HuichuanSlotAd;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct HuichuanResponse {
    pub code: String,
    pub sid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub slot_ad: Vec<HuichuanSlotAd>,
}
