use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
#[derive(prost::Message)]
pub struct TengmeiMaterial {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="1")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="2")]
    pub desc: Option<String>,
    #[prost(string, repeated, tag="3")]
    pub image_urls: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="4")]
    pub video_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="5")]
    pub video_cover_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="6")]
    pub video_duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="7")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="8")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="9")]
    pub height: Option<i32>,
}
