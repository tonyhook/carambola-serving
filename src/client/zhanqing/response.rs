use serde::{Deserialize, Serialize};

use super::ZhanqingAd;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct ZhanqingResponse {
    pub status: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads: Option<Vec<ZhanqingAd>>,
}
