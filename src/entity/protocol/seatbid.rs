use serde::{Deserialize, Serialize};

use super::Bid;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Seatbid {
   pub bid: Vec<Bid>,
}
