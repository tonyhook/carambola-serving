use serde::{Deserialize, Serialize};

use super::FwbSeatbid;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct FwbResponse {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidid: Option<String>,
    pub seatbid: FwbSeatbid,
}
