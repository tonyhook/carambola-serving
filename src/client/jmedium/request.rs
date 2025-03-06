use serde::{Deserialize, Serialize};

use super::{JmediumApp, JmediumCaid, JmediumDeal, JmediumDevice, JmediumDeviceId, JmediumExt, JmediumGeo, JmediumNetwork, JmediumSlot, JmediumUser};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct JmediumRequest {
    #[serde(rename(deserialize = "requestId", serialize = "requestId"))]
    pub request_id: String,
    pub slot: JmediumSlot,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deal: Option<JmediumDeal>,
    pub app: JmediumApp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<JmediumUser>,
    pub device: JmediumDevice,
    #[serde(rename(deserialize = "deviceId", serialize = "deviceId"))]
    pub device_id: JmediumDeviceId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caids: Option<Vec<JmediumCaid>>,
    pub network: JmediumNetwork,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<JmediumGeo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<JmediumExt>,
}
