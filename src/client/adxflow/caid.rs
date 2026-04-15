use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AdxflowCaid {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid_md5: Option<String>,
}
