use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct FanglinPos {
    pub pid: String,
    pub width: i32,
    pub height: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_floor: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num: Option<i32>,
}
