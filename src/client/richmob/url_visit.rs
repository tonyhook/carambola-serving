use serde::{Deserialize, Serialize};

use super::RichmobUrlHeader;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct RichmobUrlVisit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<RichmobUrlHeader>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
