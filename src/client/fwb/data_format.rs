use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct FwbDataFormat {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub datatype: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub len: Option<i32>,
}
