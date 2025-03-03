use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MygolbsRequest {
    pub traceid: String,
    #[serde(rename(deserialize = "appId", serialize = "appId"))]
    pub app_id: String,
    pub pid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appname: Option<String>,
    #[serde(rename(deserialize = "bundleId", serialize = "bundleId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appversion: Option<String>,
    #[serde(rename(deserialize = "appversionCode", serialize = "appversionCode"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appversion_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appstoreversion: Option<i32>,
    #[serde(rename(deserialize = "bidFloor", serialize = "bidFloor"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_floor: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsetime: Option<i32>,
    #[serde(rename(deserialize = "verCodeOfAG", serialize = "verCodeOfAG"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver_code_of_ag: Option<String>,
    #[serde(rename(deserialize = "verCodeOfHms", serialize = "verCodeOfHms"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver_code_of_hms: Option<String>,
    pub ppi: i32,
    pub screendensity: f64,
    pub nw: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub androidid: Option<String>,
    #[serde(rename(deserialize = "androididMd5", serialize = "androididMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub androidid_md5: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid: Option<String>,
    #[serde(rename(deserialize = "caidVersion", serialize = "caidVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid_version: Option<String>,
    #[serde(rename(deserialize = "caidOld", serialize = "caidOld"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid_old: Option<String>,
    #[serde(rename(deserialize = "caidOldVersion", serialize = "caidOldVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid_old_version: Option<String>,
    #[serde(rename(deserialize = "idfaMd5", serialize = "idfaMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfa_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfv: Option<String>,
    pub ua: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    pub vendor: String,
    pub devicetype: String,
    pub sv: String,
    pub s: String,
    pub w: i32,
    pub h: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adw: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adh: Option<i32>,
    pub ip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lng: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<String>,
    pub carrier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isdeeplink: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isul: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_mark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_mark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(rename(deserialize = "updateTime", serialize = "updateTime"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(rename(deserialize = "apiVersion", serialize = "apiVersion"))]
    pub api_version: i32,
    #[serde(rename(deserialize = "romVersion", serialize = "romVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rom_version: Option<String>,
    pub orientation: i32,
    #[serde(rename(deserialize = "memorySize", serialize = "memorySize"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i64>,
    #[serde(rename(deserialize = "timeZone", serialize = "timeZone"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(rename(deserialize = "modelCode", serialize = "modelCode"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_code: Option<String>,
    #[serde(rename(deserialize = "diskSize", serialize = "diskSize"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lmt: Option<i32>,
    #[serde(rename(deserialize = "phoneName", serialize = "phoneName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_name: Option<String>,
    #[serde(rename(deserialize = "osUpdateTimeSecond", serialize = "osUpdateTimeSecond"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_update_time_second: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub al: Option<String>,
    #[serde(rename(deserialize = "supportVideo", serialize = "supportVideo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_video: Option<i32>,
}
