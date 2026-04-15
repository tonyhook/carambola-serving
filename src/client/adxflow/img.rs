use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AdxflowImg {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,
}
