use serde::{Deserialize, Serialize};

use super::{KkmhApp, KkmhDevice, KkmhImp, KkmhUser};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct KkmhRequest {
    pub id: String,
    pub imps: Vec<KkmhImp>,
    pub app: KkmhApp,
    pub device: KkmhDevice,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<KkmhUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmax: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_https: Option<i32>,
}
