use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AdxworkTrack {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttype: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}
