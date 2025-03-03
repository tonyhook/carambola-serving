use serde::{Deserialize, Serialize};

use super::Display;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Ad {
    pub id: String,
    pub display: Display,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advertiser: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advertisericon: Option<String>,
}
