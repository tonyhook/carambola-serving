use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct HuoliApp {
    #[serde(rename(deserialize = "appId", serialize = "appId"))]
    pub app_id: String,
    pub name: String,
    pub bundle: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
