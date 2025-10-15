use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
#[derive(prost::Message)]
pub struct JinmoImp {
    #[prost(string, tag="1")]
    pub imp_id: String,
    #[prost(string, tag="2")]
    pub space_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="3")]
    pub keyword: Option<String>,
    #[prost(string, repeated, tag="4")]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="5")]
    pub space_width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="6")]
    pub space_height: Option<i32>,
    #[prost(int32, repeated, tag="7")]
    pub material_type: Vec<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="8")]
    pub ad_slot_type: Option<i32>,
    #[prost(int32, repeated, tag="9")]
    pub interaction_type: Vec<i32>,
    #[prost(int32, tag="10")]
    pub bid_type: i32,
    #[prost(int64, tag="11")]
    pub cpm_bid_floor: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int64, optional, tag="12")]
    pub cpc_bid_floor: Option<i64>,
}
