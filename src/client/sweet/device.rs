use serde::{Deserialize, Serialize};

use super::{SweetCaidList, SweetGeo, SweetNetwork};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct SweetDevice {
    pub ip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,
    pub ua: String,
    pub os: String,
    pub osv: String,
    #[serde(rename(deserialize = "deviceType", serialize = "deviceType"))]
    pub device_type: i32,
    pub geo: SweetGeo,
    pub network: SweetNetwork,
    pub brand: String,
    pub model: String,
    #[serde(rename(deserialize = "modelCode", serialize = "modelCode"))]
    pub model_code: String,
    pub orientation: i32,
    pub dw: i32,
    pub dh: i32,
    pub density: f64,
    pub ppi: i32,
    #[serde(rename(deserialize = "screenSize", serialize = "screenSize"))]
    pub screen_size: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serialno: Option<String>,
    #[serde(rename(deserialize = "anId", serialize = "anId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub an_id: Option<String>,
    #[serde(rename(deserialize = "anIdMd5", serialize = "anIdMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub an_id_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imei: Option<String>,
    #[serde(rename(deserialize = "imeiMd5", serialize = "imeiMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imei_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oaid: Option<String>,
    #[serde(rename(deserialize = "oaidMd5", serialize = "oaidMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oaid_md5: Option<String>,
    #[serde(rename(deserialize = "apiLevel", serialize = "apiLevel"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfa: Option<String>,
    #[serde(rename(deserialize = "idfaMd5", serialize = "idfaMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfa_md5: Option<String>,
    #[serde(rename(deserialize = "caidList", serialize = "caidList"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid_list: Option<Vec<SweetCaidList>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfv: Option<String>,
    #[serde(rename(deserialize = "idfvMd5", serialize = "idfvMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfv_md5: Option<String>,
    #[serde(rename(deserialize = "openUdid", serialize = "openUdid"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_udid: Option<String>,
    #[serde(rename(deserialize = "deviceName", serialize = "deviceName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename(deserialize = "deviceNameMd5", serialize = "deviceNameMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename(deserialize = "romVer", serialize = "romVer"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rom_ver: Option<String>,
    #[serde(rename(deserialize = "sysComplingTime", serialize = "sysComplingTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_compling_time: Option<String>,
    #[serde(rename(deserialize = "bootTime", serialize = "bootTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_time: Option<i32>,
    #[serde(rename(deserialize = "updateTime", serialize = "updateTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i32>,
    #[serde(rename(deserialize = "initTime", serialize = "initTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_time: Option<String>,
    #[serde(rename(deserialize = "diskSize", serialize = "diskSize"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<i32>,
    #[serde(rename(deserialize = "memorySize", serialize = "memorySize"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i32>,
    #[serde(rename(deserialize = "batteryStatus", serialize = "batteryStatus"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_status: Option<i32>,
    #[serde(rename(deserialize = "batteryPower", serialize = "batteryPower"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_power: Option<i32>,
    #[serde(rename(deserialize = "cpuNum", serialize = "cpuNum"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_num: Option<i32>,
    #[serde(rename(deserialize = "cpuFre", serialize = "cpuFre"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_fre: Option<f64>,
    #[serde(rename(deserialize = "timeZone", serialize = "timeZone"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    pub lmt: Option<i32>,
    pub laccu: Option<i32>,
    #[serde(rename(deserialize = "bootMark", serialize = "bootMark"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_mark: Option<String>,
    #[serde(rename(deserialize = "updateMark", serialize = "updateMark"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_mark: Option<String>,
    #[serde(rename(deserialize = "appStoreVer", serialize = "appStoreVer"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_store_ver: Option<String>,
    #[serde(rename(deserialize = "hmsVer", serialize = "hmsVer"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hms_ver: Option<String>,
    #[serde(rename(deserialize = "skadnetworkVer", serialize = "skadnetworkVer"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skadnetwork_ver: Option<Vec<String>>,
    #[serde(rename(deserialize = "installedApp", serialize = "installedApp"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_app: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t8: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[serde(rename(deserialize = "caidVendor", serialize = "caidVendor"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid_vendor: Option<i32>,
    #[serde(rename(deserialize = "bootTimeNano", serialize = "bootTimeNano"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_time_nano: Option<String>,
    #[serde(rename(deserialize = "updateTimeNano", serialize = "updateTimeNano"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time_nano: Option<String>,
}
