use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(prost::Message)]
pub struct JinmoVideo {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="1")]
    pub cover_image: Option<String>,
    #[prost(string, tag="2")]
    pub video_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="3")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="4")]
    pub video_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int64, optional, tag="5")]
    pub url_expires: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="6")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="7")]
    pub height: Option<i32>,
}
