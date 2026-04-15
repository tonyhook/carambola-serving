use serde::{Deserialize, Serialize};

use super::AdxworkAd;

#[derive(Serialize, Deserialize)]
pub struct AdxworkResponse {
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads: Option<Vec<AdxworkAd>>,
}
