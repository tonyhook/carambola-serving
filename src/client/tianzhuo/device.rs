use serde::{Deserialize, Serialize};

use super::TianzhuoGeo;

#[derive(Serialize, Deserialize)]
pub struct TianzhuoDevice {
    pub ua: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<TianzhuoGeo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devicetype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub av: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mccmnc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geofetch: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppi: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pxratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpr: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub js: Option<i32>,
    pub carrier: String,
    pub connectiontype: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gaid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imei: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imei_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imei_sha1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_id_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_id_sha1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oaid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oaid_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfa_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfa_sha1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid_version2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_sha1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_udid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flashver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "phoneName", serialize = "phoneName"))]
    pub phone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "initTimeSec", serialize = "initTimeSec"))]
    pub init_time_sec: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "initTimeFileSec", serialize = "initTimeFileSec"))]
    pub init_time_file_sec: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "bootTimeSec", serialize = "bootTimeSec"))]
    pub boot_time_sec: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "bootTimeMilliSec", serialize = "bootTimeMilliSec"))]
    pub boot_time_milli_sec: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "osUpdateTimeSec", serialize = "osUpdateTimeSec"))]
    pub os_update_time_sec: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "osUpdateTimeNanoSec", serialize = "osUpdateTimeNanoSec"))]
    pub os_update_time_nano_sec: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "updateMark", serialize = "updateMark"))]
    pub update_mark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "bootMark", serialize = "bootMark"))]
    pub boot_mark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "diskSize", serialize = "diskSize"))]
    pub disk_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "memorySize", serialize = "memorySize"))]
    pub memory_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "cpuNumber", serialize = "cpuNumber"))]
    pub cpu_number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "modelCode", serialize = "modelCode"))]
    pub model_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "timeZone", serialize = "timeZone"))]
    pub time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware_machine: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appstore_ver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vercodeofhms: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<String>,
}
