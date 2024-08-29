use serde::{Deserialize, Serialize};

use super::YibaAd;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct YibaResponse {
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads: Option<Vec<YibaAd>>,
}
