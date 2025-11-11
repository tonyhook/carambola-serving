use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MvpmobApp {
    #[serde(rename(deserialize = "bundleId", serialize = "bundleId"))]
    pub bundle_id: String,
    #[serde(rename(deserialize = "storeUrl", serialize = "storeUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_url: Option<String>,
    #[serde(rename(deserialize = "appName", serialize = "appName"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
}
