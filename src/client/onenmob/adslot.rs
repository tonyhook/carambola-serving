use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OnenmobAdslot {
    #[serde(rename(deserialize = "adType", serialize = "adType"))]
    pub ad_type: i32,
    pub position: i32,
    #[serde(rename(deserialize = "acceptedCreativeTypes", serialize = "acceptedCreativeTypes"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_creative_types: Option<i32>,
    #[serde(rename(deserialize = "acceptedInteractionType", serialize = "acceptedInteractionType"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_interaction_type: Option<i32>,
    pub width: i32,
    pub height: i32,
}
