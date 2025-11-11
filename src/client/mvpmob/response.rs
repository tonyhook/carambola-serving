use serde::{Deserialize, Serialize};

use super::{MvpmobImpResponse, MvpmobAd};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MvpmobResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_time_ms: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impr_expiration_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_expiration_timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imp: Option<MvpmobImpResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad: Option<MvpmobAd>,
}
