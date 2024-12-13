use serde::{Deserialize, Serialize};

use super::FwbDeal;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct FwbPmp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deals: Option<Vec<FwbDeal>>,
}
