use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct HuoliImp {
    pub id: i32,
    pub pid: String,
    pub width: i32,
    pub height: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "slotName", serialize = "slotName"))]
    pub slot_num: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dealid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_floor: Option<i32>,
}
