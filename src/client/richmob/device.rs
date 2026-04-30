use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct RichmobDevice {
    #[serde(rename(deserialize = "deviceId", serialize = "deviceId"))]
    pub device_id: String,
    #[serde(rename(deserialize = "deviceIdMd5", serialize = "deviceIdMd5"))]
    pub device_id_md5: String,
    pub imei: String,
    #[serde(rename(deserialize = "imeiMd5", serialize = "imeiMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imei_md5: Option<String>,
    pub oaid: String,
    #[serde(rename(deserialize = "oaidMd5", serialize = "oaidMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oaid_md5: Option<String>,
    #[serde(rename(deserialize = "openUdid", serialize = "openUdid"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_udid: Option<String>,
    pub ssid: String,
    #[serde(rename(deserialize = "wifiMac", serialize = "wifiMac"))]
    pub wifi_mac: String,
    #[serde(rename(deserialize = "phoneName", serialize = "phoneName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_name: Option<String>,
    #[serde(rename(deserialize = "phoneNameMd5", serialize = "phoneNameMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_name_md5: Option<String>,
    #[serde(rename(deserialize = "powerOnTime", serialize = "powerOnTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_on_time: Option<String>,
    pub mac: String,
    #[serde(rename(deserialize = "macMd5", serialize = "macMd5"))]
    pub mac_md5: String,
    pub imsi: String,
    #[serde(rename(deserialize = "deviceType", serialize = "deviceType"))]
    pub device_type: i32,
    pub os: String,
    #[serde(rename(deserialize = "osVersion", serialize = "osVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    pub network: i32,
    #[serde(rename(deserialize = "operatorType", serialize = "operatorType"))]
    pub operator_type: i32,
    pub swidth: i32,
    pub sheight: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpi: Option<f64>,
    #[serde(rename(deserialize = "romVersion", serialize = "romVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rom_version: Option<String>,
    #[serde(rename(deserialize = "sysComplingTime", serialize = "sysComplingTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_compling_time: Option<String>,
    #[serde(rename(deserialize = "bootTimeSec", serialize = "bootTimeSec"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_time_sec: Option<i32>,
    #[serde(rename(deserialize = "bootTimeNanoSec", serialize = "bootTimeNanoSec"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_time_nano_sec: Option<String>,
    #[serde(rename(deserialize = "osUpdateTimeSec", serialize = "osUpdateTimeSec"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_update_time_sec: Option<i32>,
    #[serde(rename(deserialize = "osUpdateTimeNanoSec", serialize = "osUpdateTimeNanoSec"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_update_time_nano_sec: Option<String>,
    #[serde(rename(deserialize = "diskSize", serialize = "diskSize"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<i32>,
    #[serde(rename(deserialize = "batteryStatus", serialize = "batteryStatus"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_status: Option<i32>,
    #[serde(rename(deserialize = "batteryPower", serialize = "batteryPower"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_power: Option<i32>,
    #[serde(rename(deserialize = "memorySize", serialize = "memorySize"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i32>,
    #[serde(rename(deserialize = "cpuNum", serialize = "cpuNum"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_num: Option<i32>,
    #[serde(rename(deserialize = "cpuFrequency", serialize = "cpuFrequency"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_frequency: Option<f64>,
    #[serde(rename(deserialize = "modelCode", serialize = "modelCode"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_code: Option<String>,
    #[serde(rename(deserialize = "timeZone", serialize = "timeZone"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    pub lmt: i32,
    pub laccu: i32,
    pub caid: String,
    #[serde(rename(deserialize = "caidMd5", serialize = "caidMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid_md5: Option<String>,
    #[serde(rename(deserialize = "caidVersion", serialize = "caidVersion"))]
    pub caid_version: String,
    #[serde(rename(deserialize = "caidVendor", serialize = "caidVendor"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid_vendor: Option<i32>,
    #[serde(rename(deserialize = "bootMark", serialize = "bootMark"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_mark: Option<String>,
    #[serde(rename(deserialize = "updateMark", serialize = "updateMark"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_mark: Option<String>,
    #[serde(rename(deserialize = "appStoreVersion", serialize = "appStoreVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_store_version: Option<String>,
    #[serde(rename(deserialize = "hmsVersion", serialize = "hmsVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hms_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppi: Option<i32>,
    #[serde(rename(deserialize = "screenSize", serialize = "screenSize"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_size: Option<f64>,
    pub idfv: String,
    pub mcc: String,
    pub mnc: String,
    #[serde(rename(deserialize = "skadnetworkVersions", serialize = "skadnetworkVersions"))]
    pub skadnetwork_versions: Vec<String>,
    #[serde(rename(deserialize = "sysInitTime", serialize = "sysInitTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_init_time: Option<String>,
    #[serde(rename(deserialize = "apiLevel", serialize = "apiLevel"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<String>,
    #[serde(rename(deserialize = "appList", serialize = "appList"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mnt_id: Option<String>,
}
