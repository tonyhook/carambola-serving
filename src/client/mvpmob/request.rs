use serde::{Deserialize, Serialize};

use super::{MvpmobAppRequest, MvpmobDevice, MvpmobImpRequest, MvpmobSite, MvpmobUser};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MvpmobRequest {
    #[serde(rename(deserialize = "requestId", serialize = "requestId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    pub app: MvpmobAppRequest,
    pub site: MvpmobSite,
    pub imp: MvpmobImpRequest,
    pub device: MvpmobDevice,
    pub user: MvpmobUser,
    pub tmax: i32,
}
