use serde::{Deserialize, Serialize};

use super::{BillowlinkAdSlot, BillowlinkApp, BillowlinkDevice, BillowlinkSite, BillowlinkUser};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct BillowlinkRequest {
    #[serde(rename(deserialize = "requestID", serialize = "requestID"))]
    pub request_id: String,
    #[serde(rename(deserialize = "apiVer", serialize = "apiVer"))]
    pub api_ver: String,
    #[serde(rename(deserialize = "adSlot", serialize = "adSlot"))]
    pub ad_slot: BillowlinkAdSlot,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<BillowlinkSite>,
    pub app: BillowlinkApp,
    pub device: BillowlinkDevice,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<BillowlinkUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support302: Option<i32>,
    pub deeplink: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(rename(deserialize = "mediaTime", serialize = "mediaTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_time: Option<i64>,
    #[serde(rename(deserialize = "sspTime", serialize = "sspTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssp_time: Option<i64>,
}
