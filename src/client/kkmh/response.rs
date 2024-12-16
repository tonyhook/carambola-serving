use serde::{Deserialize, Serialize};

use super::KkmhSeatbid;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct KkmhResponse {
    pub id: String,
    pub bidid: String,
    pub seatbids: Vec<KkmhSeatbid>,
}
