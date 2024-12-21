use serde::{Deserialize, Serialize};

use super::{MobrtbAdFormat, MobrtbApp, MobrtbDevice, MobrtbUser};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MobrtbRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub version: String,
    pub ads: Vec<MobrtbAdFormat>,
    pub app: MobrtbApp,
    pub device: MobrtbDevice,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<MobrtbUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_https: Option<bool>,
}
