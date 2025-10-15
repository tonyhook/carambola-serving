use serde::{Deserialize, Serialize};

use super::{JinmoImage, JinmoVideo};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(prost::Message)]
pub struct JinmoMaterial {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="1")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="2")]
    pub sub_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="3")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="4")]
    pub material_type: Option<i32>,
    #[prost(message, repeated, tag="5")]
    pub image_list: Vec<JinmoImage>,
    #[prost(message, repeated, tag="6")]
    pub video_list: Vec<JinmoVideo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="7")]
    pub icon_image: Option<JinmoImage>,
}
