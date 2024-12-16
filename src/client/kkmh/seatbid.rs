use serde::{Deserialize, Serialize};

use super::KkmhBid;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct KkmhSeatbid {
    pub bids: Vec<KkmhBid>,
}
