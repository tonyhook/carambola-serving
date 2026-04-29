use serde::{Deserialize, Serialize};

use super::TianzhuoBid;

#[derive(Serialize, Deserialize)]
pub struct TianzhuoSeat {
    pub bid: Option<Vec<TianzhuoBid>>,
    pub seat: Option<String>,
}
