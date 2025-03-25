use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct SweetNetwork {
    #[serde(rename(deserialize = "conType", serialize = "conType"))]
    pub con_type: i32,
    pub carrier: i32,
    pub imsi: String,
    pub mcc: String,
    pub mnc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(rename(deserialize = "macMd5", serialize = "macMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
    #[serde(rename(deserialize = "wifiMac", serialize = "wifiMac"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_mac: Option<String>,
}
