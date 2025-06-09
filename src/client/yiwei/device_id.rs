use serde::{Deserialize, Serialize};

use super::YiweiCaid;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct YiweiDeviceId {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advance_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imei: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imei_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oaid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oaid_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_id_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfa_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfv_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_udid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_mac: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_mac_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caids: Option<Vec<YiweiCaid>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aaid: Option<String>,
}
