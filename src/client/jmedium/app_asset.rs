use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct JmediumAppAsset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename(deserialize = "pkgName", serialize = "pkgName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkg_name: Option<String>,
    #[serde(rename(deserialize = "pkgMd5", serialize = "pkgMd5"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkg_md5: Option<String>,
    #[serde(rename(deserialize = "iconUrl", serialize = "iconUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corporate: Option<String>,
    #[serde(rename(deserialize = "introductionInfo", serialize = "introductionInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introduction_info: Option<String>,
    #[serde(rename(deserialize = "introductionInfoUrl", serialize = "introductionInfoUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introduction_info_url: Option<String>,
    #[serde(rename(deserialize = "privacyPolicyUrl", serialize = "privacyPolicyUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<String>,
    #[serde(rename(deserialize = "permissionInfo", serialize = "permissionInfo"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_info: Option<String>,
    #[serde(rename(deserialize = "permissionUrl", serialize = "permissionUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_url: Option<String>,
    #[serde(rename(deserialize = "recordNumber", serialize = "recordNumber"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_number: Option<String>,
}
