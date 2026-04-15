use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OnenmobApp {
    #[serde(rename(deserialize = "appId", serialize = "appId"))]
    pub app_id: String,
    #[serde(rename(deserialize = "appName", serialize = "appName"))]
    pub app_name: String,
    #[serde(rename(deserialize = "packageName", serialize = "packageName"))]
    pub package_name: String,
    #[serde(rename(deserialize = "appCategory", serialize = "appCategory"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<String>,
    #[serde(rename(deserialize = "storeUrl", serialize = "storeUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_url: Option<String>,
}
