use serde::{Deserialize, Serialize};

use super::ZhanqingCaid;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct ZhanqingDevice {
    pub ip: String,
    pub net: i32,
    pub carrier: i32,
    pub ua: String,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub devicetype: i32,
    pub os: i32,
    pub osv: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imei: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imeimd5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aid_md5: Option<String>,
    pub mac: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfa_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid: Option<String>,
    pub brand: String,
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density: Option<f64>,
    pub sw: i32,
    pub sh: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub so: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oaid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oaid_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_mark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_mark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hms_ver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appstore_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_version: Option<String>,
    #[serde(rename(deserialize = "deviceStartTime", serialize = "deviceStartTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_start_time: Option<i32>,
    #[serde(rename(deserialize = "deviceNameMd5", serialize = "deviceNameMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name_md5: Option<String>,
    #[serde(rename(deserialize = "deviceName", serialize = "deviceName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename(deserialize = "sysUpdateTime", serialize = "sysUpdateTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_update_time: Option<i32>,
    #[serde(rename(deserialize = "deviceHardDisk", serialize = "deviceHardDisk"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_hard_disk: Option<i64>,
    #[serde(rename(deserialize = "deviceMemory", serialize = "deviceMemory"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_memory: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename(deserialize = "timeZone", serialize = "timeZone"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_ssid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_bssid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppi: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid: Option<String>,
    #[serde(rename(deserialize = "bootTime", serialize = "bootTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_time: Option<String>,
    #[serde(rename(deserialize = "deviceMachine", serialize = "deviceMachine"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_machine: Option<String>,
    #[serde(rename(deserialize = "deviceModel", serialize = "deviceModel"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inittime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caidver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_num: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpi: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gps_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    #[serde(rename(deserialize = "bootTimeNano", serialize = "bootTimeNano"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_time_nano: Option<String>,
    #[serde(rename(deserialize = "updateTimeNano", serialize = "updateTimeNano"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time_nano: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caidvd: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid_list: Option<Vec<ZhanqingCaid>>,
}
