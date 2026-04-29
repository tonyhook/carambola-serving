use serde::{Deserialize, Serialize};

use super::TianzhuoDeal;

#[derive(Serialize, Deserialize)]
pub struct TianzhuoPmp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deals: Option<Vec<TianzhuoDeal>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_auction: Option<i32>,
}
