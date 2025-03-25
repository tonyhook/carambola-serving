use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct SweetTracker {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub trackertype: i32,
    pub urls: Vec<String>,
}
