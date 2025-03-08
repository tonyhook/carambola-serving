use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct ZhanqingExtendTracking {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dktracking: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dlstart: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dlcomplete: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub istart: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icomplete: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkstarttracking: Option<Vec<String>>,
}
