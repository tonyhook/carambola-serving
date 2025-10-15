use serde::{Deserialize, Serialize};

use super::JinmoCaid;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(prost::Message)]
pub struct JinmoDeviceId {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="1")]
    pub imei: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="2")]
    pub imei_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="3")]
    pub android_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="4")]
    pub android_id_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="5")]
    pub oaid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="6")]
    pub oaid_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="7")]
    pub idfa: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="8")]
    pub idfa_md5: Option<String>,
    #[prost(message, repeated, tag="9")]
    pub caid_list: Vec<JinmoCaid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="10")]
    pub idfv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="11")]
    pub idfv_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="12")]
    pub paid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="13")]
    pub paid_1_4: Option<String>,
}
