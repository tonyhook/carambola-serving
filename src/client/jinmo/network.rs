use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(prost::Message)]
pub struct JinmoNetwork {
    #[prost(int32, tag="1")]
    pub connect_type: i32,
    #[prost(string, tag="2")]
    pub ipv4: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="3")]
    pub ipv6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="4")]
    pub carrier: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="5")]
    pub ssid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="6")]
    pub wifi_mac: Option<String>,
}
