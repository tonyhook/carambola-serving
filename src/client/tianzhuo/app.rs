use serde::{Deserialize, Serialize};

use super::{TianzhuoContent, TianzhuoPublisher};

#[derive(Serialize, Deserialize)]
pub struct TianzhuoApp {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sectioncat: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagecat: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storeurl: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacypolicy: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<TianzhuoPublisher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<TianzhuoContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applist: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hc_applist: Option<Vec<i32>>,
}
