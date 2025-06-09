use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct BillowlinkApp {
    #[serde(rename(deserialize = "appID", serialize = "appID"))]
    pub app_id: String,
    pub name: String,
    pub bundle: String,
    pub ver: String,
    #[serde(rename(deserialize = "storeUrl", serialize = "storeUrl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<i32>,
}
