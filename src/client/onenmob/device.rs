use serde::{Deserialize, Serialize};

use super::OnenmobCaid;

#[derive(Serialize, Deserialize)]
pub struct OnenmobDevice {
    #[serde(rename(deserialize = "deviceId", serialize = "deviceId"))]
    pub device_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imei: Option<String>,
    #[serde(rename(deserialize = "imeiMd5", serialize = "imeiMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imei_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oaid: Option<String>,
    #[serde(rename(deserialize = "openUdid", serialize = "openUdid"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_udid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
    #[serde(rename(deserialize = "wifiMac", serialize = "wifiMac"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifi_mac: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpi: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppi: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density: Option<f64>,
    #[serde(rename(deserialize = "phoneName", serialize = "phoneName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_name: Option<String>,
    #[serde(rename(deserialize = "powerOnTime", serialize = "powerOnTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_on_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
    #[serde(rename(deserialize = "deviceType", serialize = "deviceType"))]
    pub device_type: i32,
    pub os: String,
    #[serde(rename(deserialize = "osVersion", serialize = "osVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename(deserialize = "connType", serialize = "connType"))]
    pub conn_type: i32,
    #[serde(rename(deserialize = "operatorType", serialize = "operatorType"))]
    pub operator_type: i32,
    #[serde(rename(deserialize = "screenWidth", serialize = "screenWidth"))]
    pub screen_width: i32,
    #[serde(rename(deserialize = "screenHeight", serialize = "screenHeight"))]
    pub screen_height: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<i32>,
    #[serde(rename(deserialize = "romVersion", serialize = "romVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rom_version: Option<String>,
    #[serde(rename(deserialize = "sysComplingTime", serialize = "sysComplingTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_compling_time: Option<String>,
    #[serde(rename(deserialize = "bootTimeSec", serialize = "bootTimeSec"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_time_sec: Option<String>,
    #[serde(rename(deserialize = "osUpdateTimeSec", serialize = "osUpdateTimeSec"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_update_time_sec: Option<String>,
    #[serde(rename(deserialize = "batteryStatus", serialize = "batteryStatus"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_status: Option<i32>,
    #[serde(rename(deserialize = "batteryPower", serialize = "batteryPower"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_power: Option<i32>,
    #[serde(rename(deserialize = "cpuNumber", serialize = "cpuNumber"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_number: Option<i32>,
    #[serde(rename(deserialize = "cpuFrequency", serialize = "cpuFrequency"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_frequency: Option<f64>,
    #[serde(rename(deserialize = "modelCode", serialize = "modelCode"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_code: Option<String>,
    #[serde(rename(deserialize = "timeZone", serialize = "timeZone"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lmt: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub laccu: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid: Option<String>,
    #[serde(rename(deserialize = "caidVersion", serialize = "caidVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caids: Option<Vec<OnenmobCaid>>,
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
    #[serde(rename(deserialize = "apiLevel", serialize = "apiLevel"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_level: Option<String>,
    #[serde(rename(deserialize = "hardwareMachine", serialize = "hardwareMachine"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware_machine: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serialno: Option<String>,
    #[serde(rename(deserialize = "birthTime", serialize = "birthTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_time: Option<String>,
    #[serde(rename(deserialize = "elapseTime", serialize = "elapseTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapse_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename(deserialize = "physicalMemoryByte", serialize = "physicalMemoryByte"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_memory_byte: Option<String>,
    #[serde(rename(deserialize = "harddiskSizeByte", serialize = "harddiskSizeByte"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harddisk_size_byte: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<String>,
    #[serde(rename(deserialize = "appList", serialize = "appList"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_list: Option<Vec<String>>,
}
