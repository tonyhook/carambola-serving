use serde::{Deserialize, Serialize};

use super::BillowlinkGeo;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct BillowlinkDevice {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub device_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dnt: Option<i32>,
    pub ua: String,
    pub ip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,
    pub make: String,
    pub model: String,
    pub os: i32,
    pub osv: String,
    pub w: i32,
    pub h: i32,
    pub carrier: String,
    pub connection: i32,
    pub oaid: String,
    #[serde(rename(deserialize = "oaidMd5", serialize = "oaidMd5"))]
    pub oaid_md5: String,
    pub imei: String,
    #[serde(rename(deserialize = "imeiMd5", serialize = "imeiMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imei_md5: Option<String>,
    #[serde(rename(deserialize = "imeiSha1", serialize = "imeiSha1"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imei_sha1: Option<String>,
    #[serde(rename(deserialize = "androidID", serialize = "androidID"))]
    pub android_id: String,
    #[serde(rename(deserialize = "androidIDMd5", serialize = "androidIDMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_id_md5: Option<String>,
    #[serde(rename(deserialize = "androidIDSha1", serialize = "androidIDSha1"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_id_sha1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
    pub idfa: String,
    #[serde(rename(deserialize = "idfaMd5", serialize = "idfaMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfa_md5: Option<String>,
    #[serde(rename(deserialize = "idfaSha1", serialize = "idfaSha1"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfa_sha1: Option<String>,
    pub caids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(rename(deserialize = "macMd5", serialize = "macMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bssid: Option<String>,
    pub paid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aaid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<BillowlinkGeo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpi: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename(deserialize = "sysVer", serialize = "sysVer"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_ver: Option<String>,
    #[serde(rename(deserialize = "appStorePackage", serialize = "appStorePackage"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_store_package: Option<String>,
    #[serde(rename(deserialize = "hwagVer", serialize = "hwagVer"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwag_ver: Option<String>,
    #[serde(rename(deserialize = "hmsVer", serialize = "hmsVer"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hms_ver: Option<String>,
    #[serde(rename(deserialize = "hwModel", serialize = "hwModel"))]
    pub hw_model: String,
    #[serde(rename(deserialize = "hwName", serialize = "hwName"))]
    pub hw_name: String,
    #[serde(rename(deserialize = "hwNameMd5", serialize = "hwNameMd5"))]
    pub hw_name_md5: String,
    #[serde(rename(deserialize = "hwMachine", serialize = "hwMachine"))]
    pub hw_machine: String,
    #[serde(rename(deserialize = "sysMemory", serialize = "sysMemory"))]
    pub sys_memory: String,
    #[serde(rename(deserialize = "sysDisksize", serialize = "sysDisksize"))]
    pub sys_disksize: String,
    #[serde(rename(deserialize = "deviceName", serialize = "deviceName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename(deserialize = "deviceNameMd5", serialize = "deviceNameMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name_md5: Option<String>,
    #[serde(rename(deserialize = "bootMark", serialize = "bootMark"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_mark: Option<String>,
    #[serde(rename(deserialize = "updateMark", serialize = "updateMark"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_mark: Option<String>,
    #[serde(rename(deserialize = "deviceInitializeTime", serialize = "deviceInitializeTime"))]
    pub device_initialize_time: String,
    #[serde(rename(deserialize = "bootTimeSec", serialize = "bootTimeSec"))]
    pub boot_time_sec: String,
    #[serde(rename(deserialize = "osUpdateTimeSec", serialize = "osUpdateTimeSec"))]
    pub os_update_time_sec: String,
}
