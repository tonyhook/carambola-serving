use serde::{Deserialize, Serialize};

use super::SweetBid;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct SweetResponse {
    pub id: String,
    pub code: i32,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid: Option<SweetBid>,
}
