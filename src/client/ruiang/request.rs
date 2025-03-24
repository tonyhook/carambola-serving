use serde::{Deserialize, Serialize};

use super::RuiangCaid;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct RuiangRequest {
    pub id: String,
    pub ver: String,
    #[serde(rename(deserialize = "adId", serialize = "adId"))]
    pub ad_id: String,
    #[serde(rename(deserialize = "adWidth", serialize = "adWidth"))]
    pub ad_width: i32,
    #[serde(rename(deserialize = "adHeight", serialize = "adHeight"))]
    pub ad_height: i32,
    #[serde(rename(deserialize = "appName", serialize = "appName"))]
    pub app_name: String,
    #[serde(rename(deserialize = "pkgName", serialize = "pkgName"))]
    pub pkg_name: String,
    pub apv: String,
    pub make: String,
    pub brand: String,
    pub model: String,
    pub density: f64,
    pub os: i32,
    #[serde(rename(deserialize = "osVer", serialize = "osVer"))]
    pub os_ver: String,
    #[serde(rename(deserialize = "apiLevel", serialize = "apiLevel"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_level: Option<i32>,
    #[serde(rename(deserialize = "devWidth", serialize = "devWidth"))]
    pub dev_width: i32,
    #[serde(rename(deserialize = "devHeight", serialize = "devHeight"))]
    pub dev_height: i32,
    #[serde(rename(deserialize = "devOrient", serialize = "devOrient"))]
    pub dev_orient: i32,
    #[serde(rename(deserialize = "appstoreVer", serialize = "appstoreVer"))]
    pub appstore_ver: String,
    pub hmscore: String,
    #[serde(rename(deserialize = "uiVer", serialize = "uiVer"))]
    pub ui_ver: String,
    #[serde(rename(deserialize = "devType", serialize = "devType"))]
    pub dev_type: i32,
    pub carrier: i32,
    #[serde(rename(deserialize = "operatorNop", serialize = "operatorNop"))]
    pub operator_nop: String,
    #[serde(rename(deserialize = "netType", serialize = "netType"))]
    pub net_type: i32,
    pub imsi: String,
    pub mac: String,
    #[serde(rename(deserialize = "bidPrice", serialize = "bidPrice"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<i32>,
    pub imei: String,
    #[serde(rename(deserialize = "imeiMd5", serialize = "imeiMd5"))]
    pub imei_md5: String,
    #[serde(rename(deserialize = "androidId", serialize = "androidId"))]
    pub android_id: String,
    pub oaid: String,
    #[serde(rename(deserialize = "userAgent", serialize = "userAgent"))]
    pub user_agent: String,
    pub idfa: String,
    #[serde(rename(deserialize = "idfaMd5", serialize = "idfaMd5"))]
    pub idfa_md5: String,
    pub idfv: String,
    pub udid: String,
    pub ip: String,
    pub ipv6: String,
    pub ppi: i32,
    pub lon: String,
    pub lat: String,
    #[serde(rename(deserialize = "bootMark", serialize = "bootMark"))]
    pub boot_mark: String,
    #[serde(rename(deserialize = "updateMark", serialize = "updateMark"))]
    pub update_mark: String,
    pub serialno: String,
    #[serde(rename(deserialize = "romVer", serialize = "romVer"))]
    pub rom_ver: String,
    #[serde(rename(deserialize = "sysCompilingTime", serialize = "sysCompilingTime"))]
    pub sys_compiling_time: String,
    #[serde(rename(deserialize = "devNameMd5", serialize = "devNameMd5"))]
    pub dev_name_md5: String,
    #[serde(rename(deserialize = "startupTime", serialize = "startupTime"))]
    pub startup_time: String,
    #[serde(rename(deserialize = "upgradeTime", serialize = "upgradeTime"))]
    pub upgrade_time: String,
    #[serde(rename(deserialize = "sysStartUpNanoTime", serialize = "sysStartUpNanoTime"))]
    pub sys_start_up_nano_time: String,
    #[serde(rename(deserialize = "sysUpdateNanoTime", serialize = "sysUpdateNanoTime"))]
    pub sys_update_nano_time: String,
    #[serde(rename(deserialize = "devInitNanoTime", serialize = "devInitNanoTime"))]
    pub dev_init_nano_time: String,
    pub timezone: String,
    #[serde(rename(deserialize = "hardwareModel", serialize = "hardwareModel"))]
    pub hardware_model: String,
    #[serde(rename(deserialize = "hardwareMachine", serialize = "hardwareMachine"))]
    pub hardware_machine: String,
    pub memory: i64,
    #[serde(rename(deserialize = "hardDisk", serialize = "hardDisk"))]
    pub hard_disk: i64,
    #[serde(rename(deserialize = "cpuNum", serialize = "cpuNum"))]
    pub cpu_num: i32,
    #[serde(rename(deserialize = "cpuFreq", serialize = "cpuFreq"))]
    pub cpu_freq: f64,
    #[serde(rename(deserialize = "idfaPolicy", serialize = "idfaPolicy"))]
    pub idfa_policy: i32,
    #[serde(rename(deserialize = "batteryStatus", serialize = "batteryStatus"))]
    pub battery_status: i32,
    #[serde(rename(deserialize = "batteryPower", serialize = "batteryPower"))]
    pub battery_power: i32,
    #[serde(rename(deserialize = "devFileTime", serialize = "devFileTime"))]
    pub dev_file_time: String,
    #[serde(rename(deserialize = "countryCode", serialize = "countryCode"))]
    pub country_code: String,
    pub language: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<String>,
    pub pkgs: Vec<String>,
    pub caids: Vec<RuiangCaid>,
}
