use serde::{Deserialize, Serialize};

use super::{FwbApp, FwbDevice, FwbImp, FwbUser};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct FwbRequest {
    pub id: String,
    pub version: String,
    pub imp: Vec<FwbImp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<FwbApp>,
    pub device: FwbDevice,
    pub user: FwbUser,
}
