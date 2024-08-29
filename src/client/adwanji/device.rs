use serde::{Deserialize, Serialize};

use super::AdwanjiGeo;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct AdwanjiDevice {
    pub ua: String,
    pub geo: AdwanjiGeo,
    pub ip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,
    pub devicetype: i32,
    pub make: String,
    pub brand: String,
    pub model: String,
    pub os: i32,
    pub osv: String,
    pub oslevel: i32,
    pub resolution: String,
    pub sh: i32,
    pub sw: i32,
    pub ppi: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpi: Option<i32>,
    pub density: f64,
    pub orientation: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfamd5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imei: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imeimd5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aidmd5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oaid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oaidmd5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caidver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macmd5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meid: Option<String>,
    pub carrier: String,
    pub conn: i32,
    pub imsi: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkgs: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appstorever: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appstorevername: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifissid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifimac: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serialno: Option<String>,
    pub language: String,
    pub countrycode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uiver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub romver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hmsver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwagver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compilingtime: Option<String>,
    pub starttime: String,
    pub startnanotime: String,
    pub startmilltime: String,
    pub birthtime: String,
    pub osupdatetime: String,
    pub osupdatenanotime: String,
    pub hwname: String,
    pub hwmodel: String,
    pub hwmachine: String,
    pub sysmemory: String,
    pub sysdisksize: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpunum: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpufreq: Option<String>,
    pub timezone: String,
    pub updatemark: String,
    pub bootmark: String,
    #[serde(rename(deserialize = "batteryStatus", serialize = "batteryStatus"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_status: Option<i32>,
    #[serde(rename(deserialize = "batteryPower", serialize = "batteryPower"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub battery_power: Option<i32>,
    #[serde(rename(deserialize = "idfaPolicy", serialize = "idfaPolicy"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfa_policy: Option<i32>,
    #[serde(rename(deserialize = "preCaid", serialize = "preCaid"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_caid: Option<String>,
    #[serde(rename(deserialize = "preCaidVersion", serialize = "preCaidVersion"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_caid_version: Option<String>,
    #[serde(rename(deserialize = "screenSize", serialize = "screenSize"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reffer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caid_vendor: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastcaid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastcaidver: Option<String>,
}
