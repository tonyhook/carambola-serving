use serde::{Deserialize, Serialize};

use super::AdxflowSeatbid;

#[derive(Serialize, Deserialize)]
pub struct AdxflowResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seatbid: Option<Vec<AdxflowSeatbid>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<String>,
}
