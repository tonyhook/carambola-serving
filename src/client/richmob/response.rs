use serde::{Deserialize, Serialize};

use super::RichmobAdv;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct RichmobResponse {
    pub code: i32,
    pub msg: String,
    #[serde(rename(deserialize = "requestId", serialize = "requestId"))]
    pub request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adv: Option<RichmobAdv>,
    #[serde(rename(deserialize = "expirationTime", serialize = "expirationTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<i32>,
}
