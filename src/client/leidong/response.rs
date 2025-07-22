use serde::{Deserialize, Serialize};

use super::{LeidongImp, LeidongAd};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct LeidongResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_time_ms: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impr_expiration_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_expiration_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imp: Option<LeidongImp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad: Option<LeidongAd>,
}
