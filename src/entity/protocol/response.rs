use serde::{Deserialize, Serialize};

use super::Seatbid;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Response {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbr: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seatbid: Option<Vec<Seatbid>>,
}
