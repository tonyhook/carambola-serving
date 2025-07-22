use serde::{Deserialize, Serialize};

use super::{LeidongAppRequest, LeidongDevice, LeidongImp, LeidongSite, LeidongUser};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct LeidongRequest {
    #[serde(rename(deserialize = "requestId", serialize = "requestId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    pub app: LeidongAppRequest,
    pub site: LeidongSite,
    pub imp: LeidongImp,
    pub device: LeidongDevice,
    pub user: LeidongUser,
    pub tmax: i32,
}
