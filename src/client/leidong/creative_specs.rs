use serde::{Deserialize, Serialize};

use super::LeidongSize;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct LeidongCreativeSpecs {
    #[serde(rename(deserialize = "creativeType", serialize = "creativeType"))]
    pub support_creative_type: i32,
    #[serde(rename(deserialize = "supportSizes", serialize = "supportSizes"))]
    pub support_sizes: LeidongSize,
}
