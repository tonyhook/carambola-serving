use serde::{Deserialize, Serialize};

use super::TianzhuoSegment;

#[derive(Serialize, Deserialize)]
pub struct TianzhuoData {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment: Option<Vec<TianzhuoSegment>>,
}
