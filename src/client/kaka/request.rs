use serde::{Deserialize, Serialize};

use super::{KakaApp, KakaDevice, KakaGeo, KakaImp, KakaUser};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct KakaRequest {
    pub request_id: String,
    pub api_version: String,
    pub device: KakaDevice,
    pub app: KakaApp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<KakaGeo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<KakaUser>,
    pub imps: Vec<KakaImp>,
}
