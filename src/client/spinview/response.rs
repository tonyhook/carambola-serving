use serde::{Deserialize, Serialize};

use super::SpinviewAd;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct SpinviewResponse {
    pub id: String,
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads: Option<Vec<SpinviewAd>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cur: Option<String>,
}
