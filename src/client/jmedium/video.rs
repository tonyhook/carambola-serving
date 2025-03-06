use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct JmediumVideo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename(deserialize = "coverUrl", serialize = "coverUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename(deserialize = "forceDuration", serialize = "forceDuration"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename(deserialize = "endUrlType", serialize = "endUrlType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_url_type: Option<i32>,
    #[serde(rename(deserialize = "endUrl", serialize = "endUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_url: Option<String>,
}
