use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct JmediumNetwork {
    #[serde(rename(deserialize = "userAgent", serialize = "userAgent"))]
    pub user_agent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,
    pub mac: String,
    #[serde(rename(deserialize = "macMd5", serialize = "macMd5"))]
    pub mac_md5: String,
    #[serde(rename(deserialize = "macSha1", serialize = "macSha1"))]
    pub mac_sha1: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bssid: Option<String>,
    pub carrier: i32,
    #[serde(rename(deserialize = "networkType", serialize = "networkType"))]
    pub network_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}
