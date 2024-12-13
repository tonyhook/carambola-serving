use serde::{Deserialize, Serialize};

use super::FanglinAd;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct FanglinResponse {
    pub req_id: String,
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<FanglinAd>>,
}
