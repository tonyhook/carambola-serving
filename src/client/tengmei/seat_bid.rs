use serde::{Deserialize, Serialize};

use super::TengmeiBid;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
#[derive(prost::Message)]
pub struct TengmeiSeatBid {
    #[prost(string, tag="1")]
    pub seat: String,
    #[prost(message, repeated, tag="2")]
    pub bid: Vec<TengmeiBid>,
}
