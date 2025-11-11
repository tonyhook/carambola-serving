use serde::{Deserialize, Serialize};

use super::MvpmobSize;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MvpmobCreativeSpecs {
    #[serde(rename(deserialize = "creativeType", serialize = "creativeType"))]
    pub support_creative_type: i32,
    #[serde(rename(deserialize = "supportSizes", serialize = "supportSizes"))]
    pub support_sizes: MvpmobSize,
}
