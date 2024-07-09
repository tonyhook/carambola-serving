use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct DataAsset {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<i32>,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datatype: Option<i32>,
}
