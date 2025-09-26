use serde::{Deserialize, Serialize};

use super::KakaAcceptedSize;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct KakaImp {
    pub id: String,
    #[serde(rename = "adType")]
    pub ad_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos: Option<i32>,
    #[serde(rename = "acceptedSize")]
    pub accepted_size: Vec<KakaAcceptedSize>,
    #[serde(rename = "acceptedCreativeTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_creative_types: Option<i32>,
    #[serde(rename = "acceptedInteractionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_interaction_type: Option<i32>,
    #[serde(rename = "bidFloor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_floor: Option<i32>,
}
