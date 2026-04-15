use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AdxflowBidinfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidfloor: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidtype: Option<i32>,
}
