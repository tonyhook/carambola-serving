use serde::{Deserialize, Serialize};

use super::Placement;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Item {
    pub id: String,
    pub spec: Placement,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flr: Option<i32>,
}
