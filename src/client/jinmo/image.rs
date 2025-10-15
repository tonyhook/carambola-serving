use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(prost::Message)]
pub struct JinmoImage {
    #[prost(string, tag="1")]
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="2")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="3")]
    pub height: Option<i32>,
}
