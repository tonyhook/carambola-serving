use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct RichmobMonitor {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub monitortype: i32,
    pub urls: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename(deserialize = "contentType", serialize = "contentType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename(deserialize = "reportContent", serialize = "reportContent"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_content: Option<String>,
}
