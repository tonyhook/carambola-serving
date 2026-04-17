use serde::{Deserialize, Serialize};

use super::TengmeiSeatBid;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
#[derive(prost::Message)]
pub struct TengmeiResponse {
    #[prost(string, tag="1")]
    pub id: String,
    #[prost(int32, tag="2")]
    pub status_code: i32,
    #[prost(string, tag="3")]
    pub msg: String,
    #[prost(message, repeated, tag="4")]
    pub seat_bid: Vec<TengmeiSeatBid>,
}
