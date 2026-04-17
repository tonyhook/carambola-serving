use serde::{Deserialize, Serialize};

use super::{TengmeiAppInfo, TengmeiLinks, TengmeiMaterial, TengmeiTracking};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
#[derive(prost::Message)]
pub struct TengmeiBid {
    #[prost(string, tag="1")]
    pub id: String,
    #[prost(string, tag="2")]
    pub imp_id: String,
    #[prost(string, tag="3")]
    pub creative_id: String,
    #[prost(int32, tag="4")]
    pub price: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="10")]
    pub material: Option<TengmeiMaterial>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="11")]
    pub links: Option<TengmeiLinks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="12")]
    pub tracking: Option<TengmeiTracking>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="13")]
    pub app_info: Option<TengmeiAppInfo>,
}
