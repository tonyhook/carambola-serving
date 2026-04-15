use serde::{Deserialize, Serialize};

use super::OnenmobRequestHeader;

#[derive(Serialize, Deserialize)]
pub struct OnenmobTrack {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub track_type: i32,
    pub urls: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename(deserialize = "contentType", serialize = "contentType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename(deserialize = "requestHeader", serialize = "requestHeader"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_header: Option<Vec<OnenmobRequestHeader>>,
}
