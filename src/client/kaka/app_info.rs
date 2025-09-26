use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct KakaAppInfo {
    #[serde(rename = "iconUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "appName")]
    pub app_name: String,
    #[serde(rename = "packageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(rename = "versionName")]
    pub version_name: String,
    #[serde(rename = "companyName")]
    pub company_name: String,
    #[serde(rename = "appPermissionsUrl")]
    pub app_permissions_url: String,
    #[serde(rename = "privacyPolicyUrl")]
    pub privacy_policy_url: String,
    #[serde(rename = "downloadUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(rename = "appDescriptionUrl")]
    pub app_description_url: String,
}
