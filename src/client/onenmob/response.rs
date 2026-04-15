use serde::{Deserialize, Serialize};

use super::OnenmobAd;

#[derive(Serialize, Deserialize)]
pub struct OnenmobResponse {
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<OnenmobResponseData>,
}

#[derive(Serialize, Deserialize)]
pub struct OnenmobResponseData {
    #[serde(rename(deserialize = "requestId", serialize = "requestId"))]
    pub request_id: String,
    #[serde(rename(deserialize = "statusCode", serialize = "statusCode"))]
    pub status_code: i32,
    #[serde(rename(deserialize = "expirationTime", serialize = "expirationTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<i32>,
    #[serde(rename(deserialize = "bidPrice", serialize = "bidPrice"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads: Option<OnenmobAd>,
}
