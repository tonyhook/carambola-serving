use serde::{Deserialize, Serialize};

use super::FwbBid;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct FwbSeatbid {
   pub bid: Vec<FwbBid>,
}
