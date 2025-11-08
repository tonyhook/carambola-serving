use serde::{Deserialize, Serialize};

use super::{HuoliApp, HuoliImp, HuoliDevice, HuoliUser};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct HuoliRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
    pub imp: Vec<HuoliImp>,
    pub app: HuoliApp,
    pub device: HuoliDevice,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<HuoliUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i32>,
}
