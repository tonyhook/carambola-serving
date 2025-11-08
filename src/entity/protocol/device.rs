use serde::{Deserialize, Deserializer, Serialize};

use super::Geo;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
pub struct Device {
    #[serde(rename(deserialize = "type", serialize = "type"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devicetype: Option<i32>,
    pub ua: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oslevel: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwmodel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwmachine: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub romname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub romv: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "convert_string_from_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub romtime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hmsv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uiname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uiv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skan: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysmemory: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysavailabledisksize: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysdisksize: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syscpu: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syscpufreq: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysbatterystatus: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sysbatterypower: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppi: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pxratio: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mccmnc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contype: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xff: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "convert_string_from_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boottime: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "convert_string_from_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inittime: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "convert_string_from_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updatetime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mntid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lmt: Option<i32>,
    #[serde(default)]
    #[serde(deserialize_with = "convert_string_from_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updatemark: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "convert_string_from_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bootmark: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "convert_string_from_number")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initmark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<Geo>,
}

pub fn convert_string_from_number<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Number(i64),
        Float(f64),
    }

    match StringOrNumber::deserialize(deserializer) {
        Ok(StringOrNumber::String(s)) => Ok(Some(s)),
        Ok(StringOrNumber::Number(i)) => Ok(Some(i.to_string())),
        Ok(StringOrNumber::Float(f)) => Ok(Some(f.to_string())),
        Err(_) => Ok(None),
    }
}
