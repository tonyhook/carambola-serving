use serde::{Deserialize, Serialize};

use super::KakaAd;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct KakaResponse {
    pub success: bool,
    #[serde(rename = "errorCode")]
    pub error_code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads: Option<Vec<KakaAd>>,
}
