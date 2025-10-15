use serde::{Deserialize, Serialize};

use super::JinmoImage;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(prost::Message)]
pub struct JinmoAdApp {
    #[prost(string, tag="1")]
    pub app_name: String,
    #[prost(string, tag="2")]
    pub package_name: String,
    #[prost(string, tag="3")]
    pub app_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="4")]
    pub app_desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int64, optional, tag="5")]
    pub app_size: Option<i64>,
    #[prost(string, tag="6")]
    pub app_developer: String,
    #[prost(string, tag="7")]
    pub privacy_policy_link: String,
    #[prost(string, tag="8")]
    pub permissions_link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag="9")]
    pub app_icon: Option<JinmoImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="10")]
    pub download_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int64, optional, tag="11")]
    pub download_url_expires: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="12")]
    pub file_md5: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag="13")]
    pub app_desc_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int64, optional, tag="14")]
    pub itunes_id: Option<i64>,
}
