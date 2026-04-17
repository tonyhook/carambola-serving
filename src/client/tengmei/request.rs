use serde::{Deserialize, Serialize};

use super::{TengmeiApp, TengmeiDevice, TengmeiImp, TengmeiUser};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
#[derive(prost::Message)]
pub struct TengmeiRequest {
    #[prost(string, tag="1")]
    pub id: String,
    #[prost(string, tag="2")]
    pub api_version: String,
    #[prost(message, repeated, tag="3")]
    pub imp: Vec<TengmeiImp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="4")]
    pub app: Option<TengmeiApp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="5")]
    pub device: Option<TengmeiDevice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="6")]
    pub user: Option<TengmeiUser>,
    #[prost(int32, tag="7")]
    pub timeout: i32,
}
