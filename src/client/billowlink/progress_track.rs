use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct BillowlinkProgressTrack {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<Vec<String>>,
    #[serde(rename(deserialize = "firstQuartile", serialize = "firstQuartile"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_quartile: Option<Vec<String>>,
    #[serde(rename(deserialize = "midPoint", serialize = "midPoint"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mid_point: Option<Vec<String>>,
    #[serde(rename(deserialize = "thirdQuartile", serialize = "thirdQuartile"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_quartile: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete: Option<Vec<String>>,
}
