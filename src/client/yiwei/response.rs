use serde::{Deserialize, Serialize};

use super::YiweiAd;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct YiweiResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads: Option<Vec<YiweiAd>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_expire_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imp_expire_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_expire_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_block_time: Option<String>,
}
