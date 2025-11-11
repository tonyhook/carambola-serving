use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MvpmobAppList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename(deserialize = "base64Flag", serialize = "base64Flag"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base64_flag: Option<bool>,
}
