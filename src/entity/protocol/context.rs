use serde::{Deserialize, Serialize};

use super::{App, Device, Site, User};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Context {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<App>,
    pub device: Device,
    pub user: User,
}
