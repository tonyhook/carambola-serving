use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct ZhanqingPlaypercentage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}
