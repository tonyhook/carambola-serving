use serde::{Deserialize, Serialize};

use super::{SweetApp, SweetDevice, SweetImp, SweetUser};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct SweetRequest {
    pub id: String,
    pub ver: String,
    pub imp: SweetImp,
    pub app: SweetApp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<SweetUser>,
    pub device: SweetDevice,
}
