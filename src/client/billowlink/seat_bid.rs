use serde::{Deserialize, Serialize};

use super::BillowlinkBid;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct BillowlinkSeatBid {
    pub bids: Vec<BillowlinkBid>,
}
