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
    pub os_version: String,
    pub brand: String,
    pub model: String,
    pub language: String,
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
    pub rom_version: String,
    #[serde(rename(deserialize = "sysComplingTime", serialize = "sysComplingTime"))]
    pub sys_compling_time: String,
    #[serde(rename(deserialize = "bootTimeSec", serialize = "bootTimeSec"))]
    pub boot_time_sec: i32,
    #[serde(rename(deserialize = "bootTimeNanoSec", serialize = "bootTimeNanoSec"))]
    pub boot_time_nano_sec: String,
    #[serde(rename(deserialize = "osUpdateTimeSec", serialize = "osUpdateTimeSec"))]
    pub os_update_time_sec: i32,
    #[serde(rename(deserialize = "osUpdateTimeNanoSec", serialize = "osUpdateTimeNanoSec"))]
    pub os_update_time_nano_sec: String,
    #[serde(rename(deserialize = "diskSize", serialize = "diskSize"))]
    pub disk_size: i32,
    #[serde(rename(deserialize = "batteryStatus", serialize = "batteryStatus"))]
    pub battery_status: i32,
    #[serde(rename(deserialize = "batteryPower", serialize = "batteryPower"))]
    pub battery_power: i32,
    #[serde(rename(deserialize = "memorySize", serialize = "memorySize"))]
    pub memory_size: i32,
    #[serde(rename(deserialize = "cpuNum", serialize = "cpuNum"))]
    pub cpu_num: i32,
    #[serde(rename(deserialize = "cpuFrequency", serialize = "cpuFrequency"))]
    pub cpu_frequency: f64,
    #[serde(rename(deserialize = "modelCode", serialize = "modelCode"))]
    pub model_code: String,
    #[serde(rename(deserialize = "timeZone", serialize = "timeZone"))]
    pub time_zone: String,
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
    pub ppi: i32,
    #[serde(rename(deserialize = "screenSize", serialize = "screenSize"))]
    pub screen_size: f64,
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
