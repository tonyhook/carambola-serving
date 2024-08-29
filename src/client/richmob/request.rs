use serde::{Deserialize, Serialize};

use super::{RichmobApp, RichmobAdslot, RichmobDevice, RichmobUser};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct RichmobRequest {
    #[serde(rename(deserialize = "requestId", serialize = "requestId"))]
    pub request_id: String,
    #[serde(rename(deserialize = "docVersion", serialize = "docVersion"))]
    pub doc_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub ua: String,
    pub ip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<RichmobUser>,
    pub app: RichmobApp,
    pub device: RichmobDevice,
    pub adslot: RichmobAdslot,
    #[serde(rename(deserialize = "deepLink", serialize = "deepLink"))]
    pub deep_link: bool,
}
