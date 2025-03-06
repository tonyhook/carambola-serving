use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct JmediumDevice {
    #[serde(rename(deserialize = "osType", serialize = "osType"))]
    pub os_type: i32,
    #[serde(rename(deserialize = "type", serialize = "type"))]
    pub devicetype: i32,
    #[serde(rename(deserialize = "osVersion", serialize = "osVersion"))]
    pub os_version: String,
    #[serde(rename(deserialize = "osUiVersion", serialize = "osUiVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_ui_version: Option<String>,
    #[serde(rename(deserialize = "androidApiLevel", serialize = "androidApiLevel"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_api_level: Option<i32>,
    #[serde(rename(deserialize = "sysCompilingTime", serialize = "sysCompilingTime"))]
    pub sys_compiling_time: String,
    #[serde(rename(deserialize = "sysUpdateTime", serialize = "sysUpdateTime"))]
    pub sys_update_time: String,
    #[serde(rename(deserialize = "sysStartupTime", serialize = "sysStartupTime"))]
    pub sys_startup_time: String,
    #[serde(rename(deserialize = "sysInitTime", serialize = "sysInitTime"))]
    pub sys_init_time: String,
    #[serde(rename(deserialize = "sysUpdateTimeNanoSec", serialize = "sysUpdateTimeNanoSec"))]
    pub sys_update_time_nano_sec: String,
    #[serde(rename(deserialize = "sysStartupTimeMilliSec", serialize = "sysStartupTimeMilliSec"))]
    pub sys_startup_time_milli_sec: String,
    #[serde(rename(deserialize = "birthMark", serialize = "birthMark"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_mark: Option<String>,
    #[serde(rename(deserialize = "bootMark", serialize = "bootMark"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_mark: Option<String>,
    #[serde(rename(deserialize = "updateMark", serialize = "updateMark"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_mark: Option<String>,
    #[serde(rename(deserialize = "romVersion", serialize = "romVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rom_version: Option<String>,
    #[serde(rename(deserialize = "deviceName", serialize = "deviceName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename(deserialize = "deviceNameMd5", serialize = "deviceNameMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name_md5: Option<String>,
    #[serde(rename(deserialize = "cpuNum", serialize = "cpuNum"))]
    pub cpu_num: i32,
    #[serde(rename(deserialize = "sysDiskSize", serialize = "sysDiskSize"))]
    pub sys_disk_size: i64,
    #[serde(rename(deserialize = "sysMemorySize", serialize = "sysMemorySize"))]
    pub sys_memory_size: i64,
    pub model: String,
    #[serde(rename(deserialize = "hardwareModel", serialize = "hardwareModel"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware_model: Option<String>,
    pub language: String,
    #[serde(rename(deserialize = "timeZone", serialize = "timeZone"))]
    pub time_zone: String,
    #[serde(rename(deserialize = "hmsVersion", serialize = "hmsVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hms_version: Option<String>,
    #[serde(rename(deserialize = "harmonyOsVersion", serialize = "harmonyOsVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harmony_os_version: Option<String>,
    #[serde(rename(deserialize = "hagVersion", serialize = "hagVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hag_version: Option<String>,
    #[serde(rename(deserialize = "supportDeeplink", serialize = "supportDeeplink"))]
    pub support_deeplink: i32,
    #[serde(rename(deserialize = "supportUniversal", serialize = "supportUniversal"))]
    pub support_universal: i32,
    pub make: String,
    pub brand: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
    pub width: i32,
    pub height: i32,
    pub density: f64,
    pub dpi: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppi: Option<i32>,
    pub orientation: i32,
    #[serde(rename(deserialize = "screenSize", serialize = "screenSize"))]
    pub screen_size: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serialno: Option<String>,
}
