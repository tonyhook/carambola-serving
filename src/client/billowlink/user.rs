use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct BillowlinkUser {
    #[serde(rename(deserialize = "userID", serialize = "userID"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[serde(rename(deserialize = "appList", serialize = "appList"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_list: Option<Vec<String>>,
}
