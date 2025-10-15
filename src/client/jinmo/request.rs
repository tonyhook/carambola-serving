use serde::{Deserialize, Serialize};

use super::{JinmoDevice, JinmoImp, JinmoMediaApp, JinmoUser};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(prost::Message)]
pub struct JinmoRequest {
    #[prost(string, tag="1")]
    pub request_id: String,
    #[prost(int64, tag="2")]
    pub media_id: i64,
    #[prost(message, repeated, tag="3")]
    pub imp_list: Vec<JinmoImp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="4")]
    pub device: Option<JinmoDevice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="5")]
    pub media_app: Option<JinmoMediaApp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="6")]
    pub user: Option<JinmoUser>,
    #[prost(string, tag="7")]
    pub api_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int64, optional, tag="8")]
    pub timeout: Option<i64>,
}
