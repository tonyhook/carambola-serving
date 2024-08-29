use serde::{Deserialize, Serialize};

use super::{AdwanjiApp, AdwanjiDevice, AdwanjiImp, AdwanjiUser};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct AdwanjiRequest {
    pub id: String,
    pub imp: AdwanjiImp,
    pub app: AdwanjiApp,
    pub device: AdwanjiDevice,
    pub user: AdwanjiUser,
}
