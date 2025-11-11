use serde::{Deserialize, Serialize};

use super::MvpmobAppList;

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct MvpmobUser {
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
    pub app_list: Option<MvpmobAppList>,
    #[serde(rename(deserialize = "userCategoryList", serialize = "userCategoryList"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_category_list: Option<Vec<String>>,
}
