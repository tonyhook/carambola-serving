use serde::{Deserialize, Serialize};

use super::TengmeiHc;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
#[derive(prost::Message)]
pub struct TengmeiTracking {
    #[prost(string, repeated, tag="1")]
    pub imp_urls: Vec<String>,
    #[prost(string, repeated, tag="2")]
    pub click_urls: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="3")]
    pub win_notice_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="4")]
    pub lose_notice_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="5")]
    pub hc: Option<TengmeiHc>,
}
