use serde::{Deserialize, Serialize};

use super::{TengmeiCaid, TengmeiGeo};

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Clone)]
#[derive(prost::Message)]
pub struct TengmeiDevice {
    #[prost(string, tag="1")]
    pub ua: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="2")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="3")]
    pub ipv6: Option<String>,
    #[prost(int32, tag="4")]
    pub device_type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="5")]
    pub make: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="6")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="7")]
    pub model: Option<String>,
    #[prost(int32, tag="8")]
    pub os: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="9")]
    pub osv: Option<String>,
    #[prost(int32, tag="10")]
    pub network_type: i32,
    #[prost(int32, tag="11")]
    pub carrier: i32,
    #[prost(int32, tag="12")]
    pub width: i32,
    #[prost(int32, tag="13")]
    pub height: i32,
    #[prost(int32, tag="14")]
    pub orientation: i32,
    #[prost(float, tag="15")]
    pub density: f32,
    #[prost(int32, tag="16")]
    pub ppi: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="20")]
    pub imei: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="21")]
    pub imei_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="22")]
    pub android_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="23")]
    pub android_id_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="24")]
    pub oaid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="25")]
    pub oaid_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="30")]
    pub idfa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="31")]
    pub idfa_md5: Option<String>,
    #[prost(message, repeated, tag="32")]
    pub caids: Vec<TengmeiCaid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="33")]
    pub boot_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="34")]
    pub update_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="35")]
    pub birth_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="36")]
    pub boot_mark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="37")]
    pub update_mark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="38")]
    pub paid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="39")]
    pub paid_14: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="40")]
    pub geo: Option<TengmeiGeo>,
}
