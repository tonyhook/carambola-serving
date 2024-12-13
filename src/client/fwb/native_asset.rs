use serde::{Deserialize, Serialize};

use super::FwbAsset;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct FwbNativeAsset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<FwbAsset>>,
}
