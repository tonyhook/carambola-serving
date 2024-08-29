use serde::{Deserialize, Serialize};

use super::AdwanjiBid;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct AdwanjiResponse {
    pub id: String,
    pub code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid: Option<AdwanjiBid>,
}
