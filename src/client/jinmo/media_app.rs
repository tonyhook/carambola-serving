use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(prost::Message)]
pub struct JinmoMediaApp {
    #[prost(string, tag="1")]
    pub package_name: String,
    #[prost(string, tag="2")]
    pub app_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="3")]
    pub app_version: Option<String>,
    #[prost(int32, tag="4")]
    pub network_protocol: i32,
}
