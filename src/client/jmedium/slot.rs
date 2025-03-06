use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct JmediumSlot {
    #[serde(rename(deserialize = "adSlotId", serialize = "adSlotId"))]
    pub ad_slot_id: i32,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub slottype: i32,
    pub width: i32,
    pub height: i32,
}
