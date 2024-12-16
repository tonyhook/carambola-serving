use serde::{Deserialize, Serialize};

use super::{KkmhCaid, KkmhDeviceExt, KkmhGeo};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct KkmhDevice {
    pub ua: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<KkmhGeo>,
    pub ip: String,
    pub devt: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub os: String,
    pub osv: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hwv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppi: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ct: Option<i32>,
    pub ca: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imei: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imeimd5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oaid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idfamd5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caids: Option<Vec<KkmhCaid>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macmd5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boot_mark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_mark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_init_sec: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_start_sec: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware_machine: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_memory_byte: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harddisk_size_byte: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_update_sec: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware_model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext: Option<KkmhDeviceExt>,
}
