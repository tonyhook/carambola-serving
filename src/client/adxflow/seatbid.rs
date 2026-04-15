use serde::{Deserialize, Serialize};

use super::AdxflowBid;

#[derive(Serialize, Deserialize)]
pub struct AdxflowSeatbid {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid: Option<Vec<AdxflowBid>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<String>,
}
