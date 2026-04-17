use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
#[derive(prost::Message)]
pub struct TengmeiImp {
    #[prost(string, tag="1")]
    pub id: String,
    #[prost(string, tag="2")]
    pub slot_id: String,
    #[prost(int32, tag="3")]
    pub ad_type: i32,
    #[prost(int32, tag="4")]
    pub bid_floor: i32,
    #[prost(int32, tag="5")]
    pub width: i32,
    #[prost(int32, tag="6")]
    pub height: i32,
}
