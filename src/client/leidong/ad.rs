use serde::{Deserialize, Serialize};

use super::{LeidongInteraction, LeidongNative};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct LeidongAd {
    pub adid: String,
    pub creative_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render: Option<i32>,
    pub interaction: LeidongInteraction,
    pub native: LeidongNative,
}
