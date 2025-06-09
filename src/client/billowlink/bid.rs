use serde::{Deserialize, Serialize};

use super::{BillowlinkAppData, BillowlinkEventTrack, BillowlinkMaterialMeta};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct BillowlinkBid {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
    #[serde(rename(deserialize = "pddAdid", serialize = "pddAdid"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdd_adid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nurl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lurl: Option<String>,
    pub creative: BillowlinkMaterialMeta,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<BillowlinkAppData>,
    #[serde(rename(deserialize = "clickAction", serialize = "clickAction"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click_action: Option<i32>,
    pub landing: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename(deserialize = "sourceLogo", serialize = "sourceLogo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_logo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deeplink: Option<String>,
    #[serde(rename(deserialize = "eventTrack", serialize = "eventTrack"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_track: Option<BillowlinkEventTrack>,
    #[serde(rename(deserialize = "expirationTime", serialize = "expirationTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<i32>,
}
