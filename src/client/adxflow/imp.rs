use serde::{Deserialize, Serialize};

use super::AdxflowBidinfo;

#[derive(Serialize, Deserialize)]
pub struct AdxflowImp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub adid: String,
    pub madid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adtype: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ftype: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interact: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidinfo: Option<AdxflowBidinfo>,
}
