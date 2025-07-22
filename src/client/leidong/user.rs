use serde::{Deserialize, Serialize};

use super::LeidongAppList;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct LeidongUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(rename(deserialize = "tagId", serialize = "tagId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<Vec<String>>,
    #[serde(rename(deserialize = "appList", serialize = "appList"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_list: Option<LeidongAppList>,
    #[serde(rename(deserialize = "userCategoryList", serialize = "userCategoryList"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_category_list: Option<Vec<String>>,
    #[serde(rename(deserialize = "contentIdList", serialize = "contentIdList"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_id_list: Option<Vec<i32>>,
}
