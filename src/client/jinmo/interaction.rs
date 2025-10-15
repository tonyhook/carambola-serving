use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(prost::Message)]
pub struct JinmoInteraction {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="1")]
    pub interaction_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="2")]
    pub deeplink_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="3")]
    pub universal_link_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="4")]
    pub market_deeplink_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="5")]
    pub download_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="6")]
    pub landing_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="7")]
    pub wx_mini_program_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="8")]
    pub wx_mini_program_path: Option<String>,
}
