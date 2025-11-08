use serde::{Deserialize, Serialize};

use super::{HuoliCaid, HuoliGeo};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct HuoliDevice {
    pub ip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<HuoliGeo>,
    #[serde(rename(deserialize = "userAgent", serialize = "userAgent"))]
    pub user_agent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "deviceType", serialize = "deviceType"))]
    pub device_type: Option<i32>,
    pub make: String,
    pub brand: String,
    pub model: String,
    pub os: String,
    pub osv: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<i32>,
    pub network: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfamd5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imei: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imeimd5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oaid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oaidmd5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aidplain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aidmd5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<i32>,
    pub width: i32,
    pub height: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "pixelRatio", serialize = "pixelRatio"))]
    pub pixel_ratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppi: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hmsv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mosn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mosv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mappv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "bootMark", serialize = "bootMark"))]
    pub boot_mark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "updateMark", serialize = "updateMark"))]
    pub update_mark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "birthTime", serialize = "birthTime"))]
    pub birth_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "bootTime", serialize = "bootTime"))]
    pub boot_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "updateTime", serialize = "updateTime"))]
    pub update_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "systemMem", serialize = "systemMem"))]
    pub system_mem: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "systemDisk", serialize = "systemDisk"))]
    pub system_disk: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "countryCode", serialize = "countryCode"))]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "phoneName", serialize = "phoneName"))]
    pub phone_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caids: Option<Vec<HuoliCaid>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aaid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openudid: Option<String>,
}
