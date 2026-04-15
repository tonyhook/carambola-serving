use serde::{Deserialize, Serialize};

use super::{OnenmobAdslot, OnenmobApp, OnenmobDevice, OnenmobUser};

#[derive(Serialize, Deserialize)]
pub struct OnenmobRequest {
    #[serde(rename(deserialize = "requestId", serialize = "requestId"))]
    pub request_id: String,
    #[serde(rename(deserialize = "apiVersion", serialize = "apiVersion"))]
    pub api_version: String,
    #[serde(rename(deserialize = "sourceType", serialize = "sourceType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename(deserialize = "userAgent", serialize = "userAgent"))]
    pub user_agent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename(deserialize = "userInfoParam", serialize = "userInfoParam"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_info_param: Option<OnenmobUser>,
    #[serde(rename(deserialize = "appInfoParam", serialize = "appInfoParam"))]
    pub app_info_param: OnenmobApp,
    #[serde(rename(deserialize = "deviceInfoParam", serialize = "deviceInfoParam"))]
    pub device_info_param: OnenmobDevice,
    #[serde(rename(deserialize = "adSlotInfoParam", serialize = "adSlotInfoParam"))]
    pub ad_slot_info_param: OnenmobAdslot,
    #[serde(rename(deserialize = "isSupportDp", serialize = "isSupportDp"))]
    pub is_support_dp: bool,
}
