use serde::{Deserialize, Serialize};

use super::{HuichuanAd, HuichuanAdExtInfo};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct HuichuanSlotAd {
    pub slot_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad: Option<Vec<HuichuanAd>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_ext_info: Option<HuichuanAdExtInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_type: Option<Vec<i32>>,
}
