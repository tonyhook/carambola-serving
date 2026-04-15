use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AdxworkCaid {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,
}
