use serde::{Deserialize, Serialize};

use super::{FwbNativeAsset, FwbVideoAsset};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct FwbAdmobject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native: Option<FwbNativeAsset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<FwbVideoAsset>,
}
