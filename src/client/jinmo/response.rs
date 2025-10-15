use serde::{Deserialize, Serialize};

use super::JinmoAd;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(prost::Message)]
pub struct JinmoResponse {
    #[prost(int32, tag="1")]
    pub code: i32,
    #[prost(string, tag="2")]
    pub msg: String,
    #[prost(string, tag="3")]
    pub request_id: String,
    #[prost(string, tag="4")]
    pub response_id: String,
    #[prost(message, repeated, tag="5")]
    pub ad_list: Vec<JinmoAd>,
}
