use serde::{Deserialize, Serialize};

use super::{TianzhuoData, TianzhuoGeo};

#[derive(Serialize, Deserialize)]
pub struct TianzhuoUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyeruid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yob: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<TianzhuoGeo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<TianzhuoData>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_list: Option<Vec<String>>,
}
