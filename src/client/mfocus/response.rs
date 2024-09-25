use serde::{Deserialize, Serialize};

use super::MfocusResult;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MfocusResponse {
    pub code: String,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<MfocusResult>,
}
