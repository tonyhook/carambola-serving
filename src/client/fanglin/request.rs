use serde::{Deserialize, Serialize};

use super::{FanglinApp, FanglinDevice, FanglinGeo, FanglinNetwork, FanglinPos, FanglinUser};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct FanglinRequest {
    pub version: String,
    pub req_id: String,
    pub pos: FanglinPos,
    pub app: FanglinApp,
    pub device: FanglinDevice,
    pub network: FanglinNetwork,
    pub geo: FanglinGeo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<FanglinUser>,
    pub sup_dp: i32,
    pub protocol_type: i32,
}
