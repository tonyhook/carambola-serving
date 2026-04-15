use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AdxflowWx {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_app: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
