use serde::{Deserialize, Serialize};

use super::LeidongGeo;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct LeidongDevice {
    pub ua: String,
    pub ipv4: String,
    pub ipv6: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lmt: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfa: Option<String>,
    #[serde(rename(deserialize = "idfaMd5", serialize = "idfaMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfa_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfv: Option<String>,
    #[serde(rename(deserialize = "idfvMd5", serialize = "idfvMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfv_md5: Option<String>,
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
    #[serde(rename(deserialize = "androidId", serialize = "androidId"))]
    pub android_id: String,
    #[serde(rename(deserialize = "androidIdMd5", serialize = "androidIdMd5"))]
    pub android_id_md5: String,
    #[serde(rename(deserialize = "deviceType", serialize = "deviceType"))]
    pub device_type: i32,
    pub h: i32,
    pub w: i32,
    pub os: i32,
    pub osv: String,
    pub make: String,
    pub model: String,
    pub carrier: i32,
    #[serde(rename(deserialize = "connType", serialize = "connType"))]
    pub conn_type: i32,
    #[serde(rename(deserialize = "bootSecond", serialize = "bootSecond"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_second: Option<String>,
    #[serde(rename(deserialize = "updateSecond", serialize = "updateSecond"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_second: Option<String>,
    #[serde(rename(deserialize = "cpuFrequency", serialize = "cpuFrequency"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_frequency: Option<f64>,
    #[serde(rename(deserialize = "cpuNumber", serialize = "cpuNumber"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_number: Option<i32>,
    #[serde(rename(deserialize = "batteryPower", serialize = "batteryPower"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_power: Option<i32>,
    #[serde(rename(deserialize = "batteryStatus", serialize = "batteryStatus"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid: Option<String>,
    #[serde(rename(deserialize = "caidVersion", serialize = "caidVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid_version: Option<String>,
    #[serde(rename(deserialize = "preCaid", serialize = "preCaid"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_caid: Option<String>,
    #[serde(rename(deserialize = "preCaidVersion", serialize = "preCaidVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_caid_version: Option<String>,
    #[serde(rename(deserialize = "oldCaidVersion", serialize = "oldCaidVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_caid_version: Option<String>,
    #[serde(rename(deserialize = "countryCode", serialize = "countryCode"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename(deserialize = "phoneNameMd5", serialize = "phoneNameMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_name_md5: Option<String>,
    #[serde(rename(deserialize = "memorySize", serialize = "memorySize"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<f64>,
    #[serde(rename(deserialize = "diskSize", serialize = "diskSize"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<f64>,
    #[serde(rename(deserialize = "modelCode", serialize = "modelCode"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_code: Option<String>,
    #[serde(rename(deserialize = "timeZone", serialize = "timeZone"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<LeidongGeo>,
    #[serde(rename(deserialize = "complingTime", serialize = "complingTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compling_time: Option<String>,
    #[serde(rename(deserialize = "bootMark", serialize = "bootMark"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_mark: Option<String>,
    #[serde(rename(deserialize = "updateMark", serialize = "updateMark"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_mark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pxratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpi: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppi: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<i32>,
    #[serde(rename(deserialize = "deviceStartSec", serialize = "deviceStartSec"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_start_sec: Option<String>,
    #[serde(rename(deserialize = "systemUpdateSec", serialize = "systemUpdateSec"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_update_sec: Option<String>,
    #[serde(rename(deserialize = "hardwareMachine", serialize = "hardwareMachine"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware_machine: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aaid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(rename(deserialize = "apiLevel", serialize = "apiLevel"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_level: Option<String>,
    #[serde(rename(deserialize = "macMd5", serialize = "macMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_md5: Option<String>,
    #[serde(rename(deserialize = "hmsVer", serialize = "hmsVer"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hms_ver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maker: Option<String>,
    #[serde(rename(deserialize = "open_udid", serialize = "open_udid"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_udid: Option<String>,
    #[serde(rename(deserialize = "birthTime", serialize = "birthTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_time: Option<String>,
    #[serde(rename(deserialize = "bootTimeMillisec", serialize = "bootTimeMillisec"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_time_millisec: Option<String>,
    #[serde(rename(deserialize = "updateTimeNanosec", serialize = "updateTimeNanosec"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time_nanosec: Option<String>,
}
