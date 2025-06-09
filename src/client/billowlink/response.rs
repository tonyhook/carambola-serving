use serde::{Deserialize, Serialize};

use super::BillowlinkSeatBid;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct BillowlinkResponse {
    pub code: i32,
    #[serde(rename(deserialize = "requestID", serialize = "requestID"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename(deserialize = "seatBids", serialize = "seatBids"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seat_bids: Option<Vec<BillowlinkSeatBid>>,
}
