use serde::{Deserialize, Serialize};

use super::{JinmoDeviceId, JinmoGeo, JinmoNetwork};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(prost::Message)]
pub struct JinmoDevice {
    #[prost(int32, tag="1")]
    pub device_type: i32,
    #[prost(int32, tag="2")]
    pub os_type: i32,
    #[prost(string, tag="3")]
    pub os_version: String,
    #[prost(string, tag="4")]
    pub model: String,
    #[prost(string, tag="5")]
    pub brand: String,
    #[prost(int32, tag="6")]
    pub screen_width: i32,
    #[prost(int32, tag="7")]
    pub screen_height: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, tag="8")]
    pub device_ids: Option<JinmoDeviceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="9")]
    pub geo: Option<JinmoGeo>,
    #[prost(string, tag="10")]
    pub user_agent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="11")]
    pub network: Option<JinmoNetwork>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="12")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="13")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="14")]
    pub time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="15")]
    pub cpu_num: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int64, optional, tag="16")]
    pub system_disk_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int64, optional, tag="17")]
    pub system_available_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int64, optional, tag="18")]
    pub system_memory_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="19")]
    pub hwv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="20")]
    pub ppi: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="21")]
    pub rom_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="22")]
    pub hms_ver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="23")]
    pub hwag_ver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="24")]
    pub device_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="25")]
    pub device_name_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="26")]
    pub mac: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="27")]
    pub mac_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="28")]
    pub sys_compiling_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="29")]
    pub os_birth_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="30")]
    pub os_boot_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="31")]
    pub os_boot_mark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="32")]
    pub os_update_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="33")]
    pub os_update_mark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="34")]
    pub client_time: Option<String>,
    #[prost(string, repeated, tag="35")]
    pub installed_packages: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="36")]
    pub app_store_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="37")]
    pub imsi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="38")]
    pub battery_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="39")]
    pub battery_power: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(double, optional, tag="40")]
    pub cpu_freq: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="41")]
    pub orientation: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="42")]
    pub logical_width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="43")]
    pub logical_height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="44")]
    pub dpi: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(float, optional, tag="45")]
    pub density: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="46")]
    pub osl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="47")]
    pub serial_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="48")]
    pub hardware_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="49")]
    pub hardware_machine: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag="50")]
    pub auth_status: Option<i32>,
}
