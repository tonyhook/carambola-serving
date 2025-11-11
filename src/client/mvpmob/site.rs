use serde::{Deserialize, Serialize};

use super::MvpmobCreativeSpecs;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MvpmobSite {
    pub id: String,
    #[serde(rename(deserialize = "adType", serialize = "adType"))]
    pub ad_type: i32,
    #[serde(rename(deserialize = "creativeSpecs", serialize = "creativeSpecs"))]
    pub creative_specs: Vec<MvpmobCreativeSpecs>,
    #[serde(rename(deserialize = "supportInteractionType", serialize = "supportInteractionType"))]
    pub support_interaction_type: Vec<i32>,
    pub support302: bool,
}
