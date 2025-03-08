use serde::{Deserialize, Serialize};

use super::UstarsBid;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct UstarsResponse {
    pub ret: i32,
    pub bids: Option<Vec<UstarsBid>>,
}
