use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct LeidongAppResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename(deserialize = "privacyLink", serialize = "privacyLink"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_link: Option<String>,
    #[serde(rename(deserialize = "permissionLink", serialize = "permissionLink"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_link: Option<String>,
}
