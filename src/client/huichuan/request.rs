use serde::{Deserialize, Serialize};

use super::{HuichuanAdAppInfo, HuichuanAdDeviceInfo, HuichuanAdGpsInfo, HuichuanAdPosInfo, HuichuanAdUserInfo, HuichuanExpTags, HuichuanExtInfo, HuichuanHuichuanExtInfo, HuichuanPageInfo, HuichuanResInfo};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct HuichuanRequest {
    pub ad_device_info: HuichuanAdDeviceInfo,
    pub ad_app_info: HuichuanAdAppInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_gps_info: Option<HuichuanAdGpsInfo>,
    pub ad_pos_info: Vec<HuichuanAdPosInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_info: Option<HuichuanPageInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub res_info: Option<HuichuanResInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_info: Option<HuichuanExtInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_tags: Option<HuichuanExpTags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub huichuan_ext_info: Option<HuichuanHuichuanExtInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_version: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_user_info: Option<HuichuanAdUserInfo>,
}
