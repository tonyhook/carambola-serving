use serde::{Deserialize, Serialize};

use super::{MvpmobInteraction, MvpmobNative};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MvpmobAd {
    pub adid: String,
    pub creative_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render: Option<i32>,
    pub interaction: MvpmobInteraction,
    pub native: MvpmobNative,
}
