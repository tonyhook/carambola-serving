use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct HuoliCaid {
    pub id: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}
