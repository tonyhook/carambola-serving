use serde::{Deserialize, Serialize};

use super::HuichuanAdPosExtInfo;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct HuichuanAdPosInfo {
    pub req_cnt: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    pub media_slot_id: String,
    pub slot_id: i32,
    pub slot_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aw: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ah: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpm_floor: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_maxduration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_minduration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_pos_ext_info: Option<HuichuanAdPosExtInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_pkg: Option<Vec<String>>,
}
