use serde::{Deserialize, Serialize};

use super::Header;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Event {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub eventtype: i32,
    pub method: i32,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<Vec<Header>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
