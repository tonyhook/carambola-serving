use serde::{Deserialize, Serialize};

use super::RuiangData;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct RuiangResponse {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<RuiangData>>,
}
